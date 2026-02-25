# Frame V4 C Language Implementation Plan

**Version:** 3.0
**Date:** February 2026
**Status:** In Progress
**Approach:** Phased Implementation per `adding_new_language_backend.md`

---

## 1. Design Philosophy

**C can do everything PRT does.** The only difference is that C requires us to implement data structures that other languages provide built-in. This plan achieves **100% feature parity** with the PRT implementations.

| What PRT Has Built-in | C Implementation |
|----------------------|------------------|
| `dict` / `HashMap` / `Record` | `FrameDict` - simple hash map |
| `list` / `Vec` / `Array` | `FrameVec` - dynamic array |
| `any` / `Box<dyn Any>` / `object` | `void*` with type tags if needed |
| Classes with methods | Structs with function pointers or naming conventions |
| Garbage collection / ownership | Manual memory with clear ownership rules |

---

## 2. Implementation Phases

Following the phased approach from `adding_new_language_backend.md`, C implementation proceeds in 6 phases with validation tests at each stage.

| Phase | Feature Set | Primary Tests | Extended Tests | Status |
|-------|-------------|---------------|----------------|--------|
| 1 | Core System Structure | 01, 02, 04, 06 | core/*, interfaces/*, data_types/*, operators/* | ⬜ |
| 2 | Transitions & Lifecycle | 03, 05 | automata/*, control_flow/transition_* | ⬜ |
| 3 | Parameters & Return Values | 07, 13-16, 35 | capabilities/system_return_* | ⬜ |
| 4 | State Variables & Stack | 09-12, 20 | core/stack_*, exec_smoke/stack_* | ⬜ |
| 5 | Hierarchical State Machines | 08, 19, 29, 30 | control_flow/forward_*, systems/*forward* | ⬜ |
| 6 | Actions, Operations & Persistence | 21-26 | capabilities/actions_*, scoping/* | ⬜ |

**Advanced Tests (after Phase 6):**
- Documentation examples: 31-34
- Context features: 36-38
- Transition arguments: 17, 18

---

## 3. Phase 1: Core System Structure

**Goal:** Generate a compilable C system with basic interface methods and event routing.

### Features to Implement
- [ ] Per-system runtime types (FrameDict, FrameVec, FrameEvent, Compartment)
- [ ] System struct generation
- [ ] Constructor (`System_new`) / Destructor (`System_destroy`)
- [ ] `__kernel` method (event dispatcher)
- [ ] `__router` method (state dispatch)
- [ ] State handler functions (one per state)
- [ ] `__compartment` field (current state closure)
- [ ] Interface method wrappers (public API)

### Primary Validation Tests

| Test | Description | Key Features |
|------|-------------|--------------|
| `primary/01_minimal.fc` | Single state, single method | System instantiation, interface method, return value |
| `primary/02_interface.fc` | Multiple interface methods | Multiple handlers, method signatures |
| `primary/04_native_code.fc` | Native code pass-through | "Oceans model" - native code preserved |
| `primary/06_domain_vars.fc` | Domain variables | `domain:` section, field initialization |

### Extended Validation Tests

| Category | Tests | Validates |
|----------|-------|-----------|
| `core/simple_interface.fc` | 1 | Basic interface generation |
| `core/basic_cli_compile.fc` | 1 | CLI compilation |
| `interfaces/interface_handlers_emitted.fc` | 1 | Handler emission |
| `interfaces/interface_with_param.fc` | 1 | Parameterized interfaces |
| `data_types/*.fc` | 6 | Dict, list, string, int handling |
| `operators/*.fc` | 5 | Arithmetic, comparison, logical, ternary |

### C-Specific Implementation

#### Runtime Types (Generated Per-System)

```c
// For system "Foo", generate:
typedef struct Foo_FrameDictEntry { ... } Foo_FrameDictEntry;
typedef struct Foo_FrameDict { ... } Foo_FrameDict;
typedef struct Foo_FrameVec { ... } Foo_FrameVec;
typedef struct Foo_FrameEvent { ... } Foo_FrameEvent;
typedef struct Foo_Compartment { ... } Foo_Compartment;
typedef struct Foo { ... } Foo;
```

#### System Structure

```c
typedef struct Foo {
    // Runtime infrastructure
    Foo_Compartment* __compartment;           // Current state
    Foo_Compartment* __next_compartment;      // Deferred transition target
    Foo_FrameVec* _state_stack;               // Stack of compartments
    Foo_FrameVec* _context_stack;             // Stack of FrameContext*

    // Domain variables (system-specific)
    int max_retries;
    char* label;
} Foo;
```

#### Interface Method Pattern

```c
int Foo_compute(Foo* self, int a, int b) {
    Foo_FrameDict* params = Foo_FrameDict_new();
    Foo_FrameDict_set(params, "a", (void*)(intptr_t)a);
    Foo_FrameDict_set(params, "b", (void*)(intptr_t)b);
    Foo_FrameEvent* __e = Foo_FrameEvent_new("compute", params);

    Foo_FrameContext* __ctx = Foo_FrameContext_new(__e, NULL);
    Foo_FrameVec_push(self->_context_stack, __ctx);

    Foo__kernel(self, __e);

    Foo_FrameContext* ctx = Foo_FrameVec_pop(self->_context_stack);
    int result = (int)(intptr_t)ctx->_return;

    Foo_FrameContext_destroy(ctx);
    Foo_FrameDict_destroy(params);
    Foo_FrameEvent_destroy(__e);

    return result;
}
```

### Phase 1 Verification

```bash
cd framepiler_test_env/tests/common

# Primary tests
for test in primary/01_minimal primary/02_interface primary/04_native_code primary/06_domain_vars; do
    framec -l c ${test}.fc -o /tmp/test.c
    gcc -Wall -Wextra -o /tmp/test /tmp/test.c
    /tmp/test || echo "FAIL: $test"
done

# Extended tests
for test in core/simple_interface interfaces/interface_handlers_emitted; do
    framec -l c ${test}.fc -o /tmp/test.c
    gcc -Wall -Wextra -o /tmp/test /tmp/test.c
    /tmp/test || echo "FAIL: $test"
done
```

---

## 4. Phase 2: Transitions & Lifecycle

**Goal:** Implement state transitions with proper enter/exit handler invocation.

### Features to Implement
- [ ] Basic transition (`-> $State`)
- [ ] Enter handler (`$>`)
- [ ] Exit handler (`$<`)
- [ ] Deferred transition pattern (transition executes after handler completes)
- [ ] Kernel transition processing loop

### Primary Validation Tests

| Test | Description | Key Features |
|------|-------------|--------------|
| `primary/03_transition.fc` | Basic state transition | `-> $State`, state tracking |
| `primary/05_enter_exit.fc` | Enter/exit handlers | `$>()`, `<$()`, lifecycle order |

### Extended Validation Tests

| Category | Tests | Validates |
|----------|-------|-----------|
| `control_flow/transition_basic.fc` | 1 | Basic transition syntax |
| `control_flow/transition_basic_exec.fc` | 1 | Transition execution |
| `automata/moore_machine.fc` | 1 | Moore pattern (output on entry) |
| `automata/mealy_machine.fc` | 1 | Mealy pattern (output on transition) |

### C-Specific Implementation

#### Transition Expansion

```c
// -> $Processing
{
    Foo_Compartment* __compartment = Foo_Compartment_new("Processing");
    Foo__transition(self, __compartment);
    return;
}
```

#### Kernel with Transition Processing

```c
static void Foo__kernel(Foo* self, Foo_FrameEvent* __e) {
    Foo__router(self, __e);

    while (self->__next_compartment != NULL) {
        Foo_Compartment* next = self->__next_compartment;
        self->__next_compartment = NULL;

        // Exit current state
        Foo_FrameEvent exit_event = { "<$", self->__compartment->exit_args };
        Foo__router(self, &exit_event);

        // Switch compartment
        Foo_Compartment_destroy(self->__compartment);
        self->__compartment = next;

        // Enter new state
        Foo_FrameEvent enter_event = { "$>", self->__compartment->enter_args };
        Foo__router(self, &enter_event);
    }
}
```

### Phase 2 Verification

```bash
# Primary tests
for test in primary/03_transition primary/05_enter_exit; do
    framec -l c ${test}.fc -o /tmp/test.c
    gcc -Wall -Wextra -o /tmp/test /tmp/test.c
    /tmp/test || echo "FAIL: $test"
done

# Automata tests
for test in automata/moore_machine automata/mealy_machine; do
    framec -l c ${test}.fc -o /tmp/test.c
    gcc -Wall -Wextra -o /tmp/test /tmp/test.c
    /tmp/test || echo "FAIL: $test"
done
```

---

## 5. Phase 3: Parameters & Return Values

**Goal:** Support event parameters and interface return values.

### Features to Implement
- [ ] Event parameter passing via `__e->_parameters`
- [ ] `return expr` sugar (sets context return and exits handler)
- [ ] `system.return = expr` explicit assignment
- [ ] Interface header default return values (`method(): int = 10`)
- [ ] Context stack for reentrancy (`_context_stack`)
- [ ] `FrameContext` type with `_return` slot

### Primary Validation Tests

| Test | Description | Key Features |
|------|-------------|--------------|
| `primary/07_params.fc` | Event parameters | Parameter unpacking from `_parameters` |
| `primary/13_system_return.fc` | Return values | `return expr`, `system.return = expr` |
| `primary/14_system_return_default.fc` | Default returns | Interface header `= default` |
| `primary/15_system_return_chain.fc` | Chained returns | Return through multiple states |
| `primary/16_system_return_reentrant.fc` | Nested calls | Reentrant interface calls |
| `primary/35_return_init.fc` | Return initialization | Default values in interface |

### Extended Validation Tests

| Category | Tests | Validates |
|----------|-------|-----------|
| `capabilities/system_return_header_defaults.fc` | 1 | Interface header default values |
| `systems/interface_with_param.fc` | 1 | Parameterized interface |

### C-Specific Implementation

#### FrameContext

```c
typedef struct {
    Foo_FrameEvent* event;      // Reference to interface event
    void* _return;              // Return value slot
    Foo_FrameDict* _data;       // Call-scoped data dictionary
} Foo_FrameContext;
```

#### Parameter Unpacking

```c
// For handler: event(a: int, b: str)
if (strcmp(__e->_message, "event") == 0) {
    int a = (int)(intptr_t)Foo_FrameDict_get(__e->_parameters, "a");
    char* b = (char*)Foo_FrameDict_get(__e->_parameters, "b");
    // Handler body...
}
```

#### System Return Expansion

```c
// "return expr" -> set return value and exit
{
    Foo_FrameContext* __ctx = (Foo_FrameContext*)Foo_FrameVec_last(self->_context_stack);
    __ctx->_return = (void*)(intptr_t)expr;
    return;
}
```

### Phase 3 Verification

```bash
for test in primary/07_params primary/13_system_return primary/14_system_return_default \
            primary/15_system_return_chain primary/16_system_return_reentrant primary/35_return_init; do
    framec -l c ${test}.fc -o /tmp/test.c
    gcc -Wall -Wextra -o /tmp/test /tmp/test.c
    /tmp/test || echo "FAIL: $test"
done

# Capability tests
framec -l c capabilities/system_return_header_defaults.fc -o /tmp/test.c
gcc -Wall -Wextra -o /tmp/test /tmp/test.c
/tmp/test || echo "FAIL: capabilities/system_return_header_defaults"
```

---

## 6. Phase 4: State Variables & Stack

**Goal:** Support state-local variables and state stack for push/pop transitions.

### Features to Implement
- [ ] State variables (`$.var` syntax)
- [ ] State variable storage in `__compartment->state_vars`
- [ ] State stack (`_state_stack` field)
- [ ] `push$` - Push current compartment to stack
- [ ] `-> pop$` - Pop compartment and transition to it
- [ ] `Compartment_copy` for push operation
- [ ] State variable preservation across push/pop

### Primary Validation Tests

| Test | Description | Key Features |
|------|-------------|--------------|
| `primary/09_stack.fc` | Push/pop transitions | `push$`, `-> pop$` |
| `primary/10_state_var_basic.fc` | Basic state variables | `$.var` declaration and access |
| `primary/11_state_var_reentry.fc` | State variable reset | Variables reset on re-entry |
| `primary/12_state_var_push_pop.fc` | Variables with stack | Variables preserved on pop |
| `primary/20_transition_pop.fc` | Pop transition details | Pop transition mechanics |

### Extended Validation Tests

| Category | Tests | Validates |
|----------|-------|-----------|
| `core/stack_ops.fc` | 1 | Stack operations |
| `exec_smoke/stack_ops.fc` | 1 | Stack smoke test |
| `exec_smoke/stack_then_transition.fc` | 1 | Stack then transition |
| `control_flow/stack_then_transition_exec.fc` | 1 | Stack control flow |
| `control_flow/stack_pop_then_transition_exec.fc` | 1 | Pop then transition |

### C-Specific Implementation

#### State Variable Access

```c
// $.counter = 5
Foo_FrameDict_set(self->__compartment->state_vars, "counter", (void*)(intptr_t)5);

// $.counter (read)
int counter = (int)(intptr_t)Foo_FrameDict_get(self->__compartment->state_vars, "counter");
```

#### Push Operation

```c
// push$
Foo_FrameVec_push(self->_state_stack, Foo_Compartment_copy(self->__compartment));
```

#### Pop Transition

```c
// -> pop$
{
    Foo_Compartment* __saved = (Foo_Compartment*)Foo_FrameVec_pop(self->_state_stack);
    Foo__transition(self, __saved);
    return;
}
```

### Phase 4 Verification

```bash
for test in primary/09_stack primary/10_state_var_basic primary/11_state_var_reentry \
            primary/12_state_var_push_pop primary/20_transition_pop; do
    framec -l c ${test}.fc -o /tmp/test.c
    gcc -Wall -Wextra -o /tmp/test /tmp/test.c
    /tmp/test || echo "FAIL: $test"
done

# Stack smoke tests
for test in exec_smoke/stack_ops exec_smoke/stack_then_transition; do
    framec -l c ${test}.fc -o /tmp/test.c
    gcc -Wall -Wextra -o /tmp/test /tmp/test.c
    /tmp/test || echo "FAIL: $test"
done
```

---

## 7. Phase 5: Hierarchical State Machines

**Goal:** Support parent states and event forwarding.

### Features to Implement
- [ ] Parent state declaration (`$Child => $Parent`)
- [ ] Event forwarding (`=> $^` or `=> $Parent`)
- [ ] Transition with forward (`-> => $State`)
- [ ] Default forward (unhandled events automatically forward)
- [ ] `forward_event` field in compartment
- [ ] Kernel forward processing

### Primary Validation Tests

| Test | Description | Key Features |
|------|-------------|--------------|
| `primary/08_hsm.fc` | Basic HSM | `=> $Parent`, `=> $^` |
| `primary/19_transition_forward.fc` | Transition then forward | `-> => $State` |
| `primary/29_forward_enter_first.fc` | Forward with enter | Enter before forward |
| `primary/30_hsm_default_forward.fc` | Default forwarding | Auto-forward unhandled |

### Extended Validation Tests

| Category | Tests | Validates |
|----------|-------|-----------|
| `core/forward_parent.fc` | 1 | Basic parent forwarding |
| `control_flow/forward_*.fc` | 15+ | Various forward patterns |
| `systems/child_forwards_then_transition_exec.fc` | 1 | Child-to-parent forwarding |
| `systems/nested_parent_forward_then_transition_exec.fc` | 1 | Nested parent forwarding |

### C-Specific Implementation

#### Forward Expansion

```c
// => $^
{
    Foo__state_Parent(self, __e);
    return;
}
```

#### Transition with Forward

```c
// -> => $Target
{
    Foo_Compartment* __compartment = Foo_Compartment_new("Target");
    __compartment->forward_event = __e;  // Stash event
    Foo__transition(self, __compartment);
    return;
}
```

#### Kernel Forward Processing

```c
// In kernel, after transition:
if (next->forward_event == NULL) {
    Foo_FrameEvent enter_event = { "$>", self->__compartment->enter_args };
    Foo__router(self, &enter_event);
} else {
    Foo_FrameEvent* fwd = next->forward_event;
    next->forward_event = NULL;

    if (strcmp(fwd->_message, "$>") == 0) {
        Foo__router(self, fwd);
    } else {
        Foo_FrameEvent enter_event = { "$>", self->__compartment->enter_args };
        Foo__router(self, &enter_event);
        Foo__router(self, fwd);
    }
}
```

### Phase 5 Verification

```bash
for test in primary/08_hsm primary/19_transition_forward \
            primary/29_forward_enter_first primary/30_hsm_default_forward; do
    framec -l c ${test}.fc -o /tmp/test.c
    gcc -Wall -Wextra -o /tmp/test /tmp/test.c
    /tmp/test || echo "FAIL: $test"
done

# Forward pattern tests
ls control_flow/forward_*.fc | while read test; do
    framec -l c "$test" -o /tmp/test.c
    gcc -Wall -Wextra -o /tmp/test /tmp/test.c 2>/dev/null
    /tmp/test 2>/dev/null || echo "FAIL: $test"
done
```

---

## 8. Phase 6: Actions, Operations & Persistence

**Goal:** Support action methods, operations, and state persistence.

### Features to Implement
- [ ] Actions section (`actions:`) - internal methods
- [ ] Action wrappers (public API for actions)
- [ ] Operations section (`operations:`) - pure functions
- [ ] State parameters (`$State(param: type)`)
- [ ] State serialization (`save_state()`, `load_state()`)
- [ ] cJSON integration for persistence

### Primary Validation Tests

| Test | Description | Key Features |
|------|-------------|--------------|
| `primary/21_actions_basic.fc` | Action methods | `actions:` section |
| `primary/22_operations_basic.fc` | Operation methods | `operations:` section |
| `primary/23_persist_basic.fc` | Basic persistence | `save_state()` |
| `primary/24_persist_roundtrip.fc` | Save and restore | `load_state()` |
| `primary/25_persist_stack.fc` | Persist with stack | Stack serialization |
| `primary/26_state_params.fc` | Parameterized states | `$State(a: int)` |

### Extended Validation Tests

| Category | Tests | Validates |
|----------|-------|-----------|
| `capabilities/actions_emitted.fc` | 1 | Action method emission |
| `capabilities/actions_call_wrappers.fc` | 1 | Action wrapper generation |
| `capabilities/operations_emitted.fc` | 1 | Operation method emission |
| `scoping/function_scope.fc` | 1 | Function scoping |
| `scoping/nested_functions.fc` | 1 | Nested function scoping |

### C-Specific Implementation

#### Actions

```c
// actions: validate() { ... }
static bool Foo__action_validate(Foo* self) {
    // Action implementation
}

// Public wrapper
bool Foo_validate(Foo* self) {
    return Foo__action_validate(self);
}
```

#### Operations

```c
// operations: utility(): int { ... }
int Foo_utility(Foo* self) {
    // Operation implementation
}

// Static operation (no self)
int Foo_add(int a, int b) {
    return a + b;
}
```

#### Persistence (using cJSON)

```c
char* Foo_save_state(Foo* self) {
    cJSON* root = cJSON_CreateObject();
    cJSON_AddStringToObject(root, "state", self->__compartment->state);
    // Serialize state_vars, _state_stack, domain vars...
    char* json = cJSON_Print(root);
    cJSON_Delete(root);
    return json;  // Caller must free
}

Foo* Foo_restore_state(const char* json) {
    Foo* self = malloc(sizeof(Foo));
    cJSON* root = cJSON_Parse(json);
    // Restore compartment (NO enter event)
    // Restore domain vars, _state_stack...
    cJSON_Delete(root);
    return self;
}
```

### Phase 6 Verification

```bash
# Simple tests
for test in primary/21_actions_basic primary/22_operations_basic primary/26_state_params; do
    framec -l c ${test}.fc -o /tmp/test.c
    gcc -Wall -Wextra -o /tmp/test /tmp/test.c
    /tmp/test || echo "FAIL: $test"
done

# Persistence tests (need cJSON)
for test in primary/23_persist_basic primary/24_persist_roundtrip primary/25_persist_stack; do
    framec -l c ${test}.fc -o /tmp/test.c
    gcc -Wall -Wextra -o /tmp/test /tmp/test.c cJSON.c
    /tmp/test || echo "FAIL: $test"
done

# Capability tests
for test in capabilities/actions_emitted capabilities/operations_emitted capabilities/actions_call_wrappers; do
    framec -l c ${test}.fc -o /tmp/test.c
    gcc -Wall -Wextra -o /tmp/test /tmp/test.c
    /tmp/test || echo "FAIL: $test"
done
```

---

## 9. Advanced Tests

After completing all 6 phases, validate with documentation examples and context features.

### Documentation Examples

| Test | Description |
|------|-------------|
| `primary/31_doc_lamp_basic.fc` | Basic lamp example from docs |
| `primary/32_doc_lamp_hsm.fc` | HSM lamp example |
| `primary/33_doc_history_basic.fc` | History pattern |
| `primary/34_doc_history_hsm.fc` | HSM with history |

### Context Features

| Test | Description |
|------|-------------|
| `primary/36_context_basic.fc` | Context access (`@@.`) |
| `primary/37_context_reentrant.fc` | Reentrant context |
| `primary/38_context_data.fc` | Context data (`@@:data`) |

### Transition Arguments

| Test | Description |
|------|-------------|
| `primary/17_transition_enter_args.fc` | Enter arguments (`-> (args) $State`) |
| `primary/18_transition_exit_args.fc` | Exit arguments (`-> $State (args)`) |

### Context Syntax Expansion

```c
// @@.x -> access interface parameter
Foo_FrameContext* __ctx = (Foo_FrameContext*)Foo_FrameVec_last(self->_context_stack);
void* x = Foo_FrameDict_get(__ctx->event->_parameters, "x");

// @@:return = value
__ctx->_return = value;

// @@:data[key] = value
Foo_FrameDict_set(__ctx->_data, "key", value);
```

---

## 10. Comprehensive Test Categories

### Control Flow Tests (39 tests)

Frame statements in various control flow contexts:

| Pattern | C Considerations |
|---------|------------------|
| `if_*` | Standard C if/else |
| `while_*` | Standard C while |
| `transition_*` | Return after transition |
| `forward_*` | Return after forward |
| `stack_*` | Stack operations |

### Validator Tests (3 tests)

Terminal statement validation:

| Test | Validates |
|------|-----------|
| `validator/terminal_last_transition.fc` | Transitions must be terminal |
| `validator/terminal_last_forward.fc` | Forwards must be terminal |
| `validator/terminal_last_stack_ops.fc` | Stack ops must be terminal |

### Systems Tests (11 tests)

System-level integration tests covering complex patterns.

### Exec Smoke Tests (5 tests)

Quick validation of common patterns.

---

## 11. Runtime Data Structures

### FrameDict (Hash Map)

```c
typedef struct Foo_FrameDictEntry {
    char* key;
    void* value;
    struct Foo_FrameDictEntry* next;
} Foo_FrameDictEntry;

typedef struct {
    Foo_FrameDictEntry** buckets;
    int bucket_count;
    int size;
} Foo_FrameDict;

// API
static Foo_FrameDict* Foo_FrameDict_new(void);
static void Foo_FrameDict_set(Foo_FrameDict* d, const char* key, void* value);
static void* Foo_FrameDict_get(Foo_FrameDict* d, const char* key);
static Foo_FrameDict* Foo_FrameDict_copy(Foo_FrameDict* src);
static void Foo_FrameDict_destroy(Foo_FrameDict* d);
```

### FrameVec (Dynamic Array)

```c
typedef struct {
    void** items;
    int size;
    int capacity;
} Foo_FrameVec;

// API
static Foo_FrameVec* Foo_FrameVec_new(void);
static void Foo_FrameVec_push(Foo_FrameVec* v, void* item);
static void* Foo_FrameVec_pop(Foo_FrameVec* v);
static void* Foo_FrameVec_last(Foo_FrameVec* v);
static int Foo_FrameVec_size(Foo_FrameVec* v);
static void Foo_FrameVec_destroy(Foo_FrameVec* v);
```

### FrameEvent

```c
typedef struct {
    const char* _message;           // Event type: "$>", "<$", "methodName"
    Foo_FrameDict* _parameters;     // Event parameters
} Foo_FrameEvent;
```

### FrameContext

```c
typedef struct {
    Foo_FrameEvent* event;          // Reference to interface event
    void* _return;                  // Return value slot
    Foo_FrameDict* _data;           // Call-scoped data dictionary
} Foo_FrameContext;
```

### Compartment

```c
typedef struct Foo_Compartment {
    const char* state;                      // State name
    Foo_FrameDict* state_args;              // $State(args) parameters
    Foo_FrameDict* state_vars;              // $.varName storage
    Foo_FrameDict* enter_args;              // -> (args) $State
    Foo_FrameDict* exit_args;               // (args) -> $State
    Foo_FrameEvent* forward_event;          // -> => forwarding
    struct Foo_Compartment* parent_compartment;  // HSM parent
} Foo_Compartment;
```

---

## 12. Code Generation Changes

### Files to Modify

| File | Changes |
|------|---------|
| `framec/src/frame_c/visitors/mod.rs` | Ensure `TargetLanguage::C` exists |
| `framec/src/frame_c/v4/codegen/backends/c.rs` | Implement full C backend |
| `framec/src/frame_c/v4/codegen/system_codegen.rs` | C-specific codegen paths |
| `framec/src/frame_c/v4/pipeline/compiler.rs` | Register C target |

### Generated File Structure

Each `.fc` file generates a single self-contained `.c` file containing:

1. Standard includes (`stdlib.h`, `string.h`, `stdio.h`, `stdbool.h`, `stdint.h`)
2. Per-system runtime types (all `static` except public API)
3. System struct definition
4. Static kernel/router/transition functions
5. Static state handler functions
6. Public interface methods
7. Public constructor/destructor
8. Native preamble/postamble (including `main()` for tests)

---

## 13. Complete Validation Checklist

### Phase Completion

| Phase | Primary Tests | Extended Categories | Pass |
|-------|---------------|---------------------|------|
| 1 | 01, 02, 04, 06 | core/*, interfaces/*, data_types/*, operators/* | [ ] |
| 2 | 03, 05 | automata/*, control_flow/transition_* | [ ] |
| 3 | 07, 13-16, 35 | capabilities/system_return_* | [ ] |
| 4 | 09-12, 20 | core/stack_*, exec_smoke/stack_* | [ ] |
| 5 | 08, 19, 29, 30 | control_flow/forward_*, systems/*forward* | [ ] |
| 6 | 21-26 | capabilities/actions_*, scoping/* | [ ] |

### Advanced Validation

| Category | Tests | Pass |
|----------|-------|------|
| Doc examples | 31-34 | [ ] |
| Context features | 36-38 | [ ] |
| Transition args | 17, 18 | [ ] |
| Validator | 3 tests | [ ] |
| Control flow | 39 tests | [ ] |
| Systems | 11 tests | [ ] |
| Exec smoke | 5 tests | [ ] |

### Summary

| Level | Test Count |
|-------|------------|
| Primary (phases 1-6) | 27 |
| Advanced (docs, context, args) | 9 |
| Extended (all categories) | ~101 |
| **Total** | **~137** |

---

## 14. Success Criteria

- [ ] **~137 tests passing** (full parity with PRT)
- [ ] **No memory leaks** (Valgrind clean on all tests)
- [ ] **Compiles clean** with `-Wall -Wextra -Werror` on gcc and clang
- [ ] **Full feature parity** - every Frame construct works identically to PRT
- [ ] **Self-contained output** - each `.c` file compiles independently

---

## 15. Non-Goals (Out of Scope)

- Thread safety (same as PRT - not thread-safe by default)
- Optimization beyond correctness
- C89 compatibility (we use C11)
- Windows-specific code (POSIX assumed for now)
- Separate header files (future `@@codegen { c_header: separate }` option)

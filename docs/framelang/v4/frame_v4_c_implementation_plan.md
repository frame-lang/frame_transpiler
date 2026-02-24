# Frame V4 C Language Implementation Plan

**Version:** 2.0
**Date:** February 2026
**Status:** Planning
**Approach:** Full Parity with PRT (Python, Rust, TypeScript)

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

## 2. Runtime Generation Strategy

### 2.1 Per-System Generation (Current Approach)

Like PRT, each system generates its own runtime types with system-specific prefixes:

```c
// my_system.c - everything self-contained
typedef struct MySystem_FrameDictEntry { ... } MySystem_FrameDictEntry;
typedef struct MySystem_FrameDict { ... } MySystem_FrameDict;
typedef struct MySystem_FrameVec { ... } MySystem_FrameVec;
typedef struct MySystem_FrameEvent { ... } MySystem_FrameEvent;
typedef struct MySystem_FrameContext { ... } MySystem_FrameContext;
typedef struct MySystem_Compartment { ... } MySystem_Compartment;
typedef struct MySystem { ... } MySystem;
```

**Benefits:**
- Self-contained: each `.c` file compiles independently
- Matches PRT model exactly
- Easy validation: just compile and run each test file
- No linking complexity

**Future Option:** `@@codegen { runtime: shared }` could emit a shared `frame_runtime.h` instead. Not implemented initially.

### 2.2 Generated Runtime Types

Each system `Foo` generates these types (all prefixed with `Foo_`):

| Type | Purpose |
|------|---------|
| `Foo_FrameDictEntry` | Hash map entry (key + value + next) |
| `Foo_FrameDict` | String-keyed dictionary |
| `Foo_FrameVec` | Dynamic array |
| `Foo_FrameEvent` | Event routing object |
| `Foo_FrameContext` | Interface call context |
| `Foo_Compartment` | State closure |
| `Foo` | The system struct itself |

### 2.3 FrameDict Implementation (Generated Per-System)

For a system named `Foo`, the generated code includes:

```c
// ============================================================================
// Foo_FrameDict - String-keyed dictionary (generated per-system)
// ============================================================================

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

static unsigned int Foo_hash_string(const char* str) {
    unsigned int hash = 5381;
    int c;
    while ((c = *str++)) {
        hash = ((hash << 5) + hash) + c;
    }
    return hash;
}

static Foo_FrameDict* Foo_FrameDict_new(void) {
    Foo_FrameDict* d = malloc(sizeof(Foo_FrameDict));
    d->bucket_count = 16;
    d->buckets = calloc(d->bucket_count, sizeof(Foo_FrameDictEntry*));
    d->size = 0;
    return d;
}

static void Foo_FrameDict_set(Foo_FrameDict* d, const char* key, void* value) {
    unsigned int idx = Foo_hash_string(key) % d->bucket_count;
    Foo_FrameDictEntry* entry = d->buckets[idx];
    while (entry) {
        if (strcmp(entry->key, key) == 0) {
            entry->value = value;
            return;
        }
        entry = entry->next;
    }
    Foo_FrameDictEntry* new_entry = malloc(sizeof(Foo_FrameDictEntry));
    new_entry->key = strdup(key);
    new_entry->value = value;
    new_entry->next = d->buckets[idx];
    d->buckets[idx] = new_entry;
    d->size++;
}

static void* Foo_FrameDict_get(Foo_FrameDict* d, const char* key) {
    unsigned int idx = Foo_hash_string(key) % d->bucket_count;
    Foo_FrameDictEntry* entry = d->buckets[idx];
    while (entry) {
        if (strcmp(entry->key, key) == 0) {
            return entry->value;
        }
        entry = entry->next;
    }
    return NULL;
}

static Foo_FrameDict* Foo_FrameDict_copy(Foo_FrameDict* src) {
    Foo_FrameDict* dst = Foo_FrameDict_new();
    for (int i = 0; i < src->bucket_count; i++) {
        Foo_FrameDictEntry* entry = src->buckets[i];
        while (entry) {
            Foo_FrameDict_set(dst, entry->key, entry->value);
            entry = entry->next;
        }
    }
    return dst;
}

static void Foo_FrameDict_destroy(Foo_FrameDict* d) {
    for (int i = 0; i < d->bucket_count; i++) {
        Foo_FrameDictEntry* entry = d->buckets[i];
        while (entry) {
            Foo_FrameDictEntry* next = entry->next;
            free(entry->key);
            free(entry);
            entry = next;
        }
    }
    free(d->buckets);
    free(d);
}

// ============================================================================
// Foo_FrameVec - Dynamic array (generated per-system)
// ============================================================================

typedef struct {
    void** items;
    int size;
    int capacity;
} Foo_FrameVec;

static Foo_FrameVec* Foo_FrameVec_new(void) {
    Foo_FrameVec* v = malloc(sizeof(Foo_FrameVec));
    v->capacity = 8;
    v->size = 0;
    v->items = malloc(sizeof(void*) * v->capacity);
    return v;
}

static void Foo_FrameVec_push(Foo_FrameVec* v, void* item) {
    if (v->size >= v->capacity) {
        v->capacity *= 2;
        v->items = realloc(v->items, sizeof(void*) * v->capacity);
    }
    v->items[v->size++] = item;
}

static void* Foo_FrameVec_pop(Foo_FrameVec* v) {
    if (v->size == 0) return NULL;
    return v->items[--v->size];
}

static void* Foo_FrameVec_last(Foo_FrameVec* v) {
    if (v->size == 0) return NULL;
    return v->items[v->size - 1];
}

static int Foo_FrameVec_size(Foo_FrameVec* v) {
    return v->size;
}

static void Foo_FrameVec_destroy(Foo_FrameVec* v) {
    free(v->items);
    free(v);
}
```

---

## 3. Core Data Structures (Full Parity)

All types are prefixed with the system name (e.g., `Foo_`). Examples below use `Foo` as the system name.

### 3.1 FrameEvent

**Exactly matches PRT:**

```c
typedef struct {
    const char* _message;           // Event type: "$>", "<$", "methodName"
    Foo_FrameDict* _parameters;     // Event parameters (dict, not typed struct!)
} Foo_FrameEvent;

static Foo_FrameEvent* Foo_FrameEvent_new(const char* message, Foo_FrameDict* parameters) {
    Foo_FrameEvent* e = malloc(sizeof(Foo_FrameEvent));
    e->_message = message;
    e->_parameters = parameters;
    return e;
}

static void Foo_FrameEvent_destroy(Foo_FrameEvent* e) {
    // Note: _parameters ownership depends on context
    free(e);
}
```

### 3.2 FrameContext

**Exactly matches PRT:**

```c
typedef struct {
    Foo_FrameEvent* event;          // Reference to interface event
    void* _return;                  // Return value slot (void* for any type)
    Foo_FrameDict* _data;           // Call-scoped data dictionary
} Foo_FrameContext;

static Foo_FrameContext* Foo_FrameContext_new(Foo_FrameEvent* event, void* default_return) {
    Foo_FrameContext* ctx = malloc(sizeof(Foo_FrameContext));
    ctx->event = event;
    ctx->_return = default_return;
    ctx->_data = Foo_FrameDict_new();
    return ctx;
}

static void Foo_FrameContext_destroy(Foo_FrameContext* ctx) {
    Foo_FrameDict_destroy(ctx->_data);
    free(ctx);
}
```

### 3.3 Compartment

**Exactly matches PRT - all 6 fields + parent:**

```c
typedef struct Foo_Compartment {
    const char* state;                      // State name (string, not enum!)
    Foo_FrameDict* state_args;              // $State(args) parameters
    Foo_FrameDict* state_vars;              // $.varName storage
    Foo_FrameDict* enter_args;              // -> (args) $State
    Foo_FrameDict* exit_args;               // (args) -> $State
    Foo_FrameEvent* forward_event;          // -> => forwarding
    struct Foo_Compartment* parent_compartment;  // HSM parent
} Foo_Compartment;

static Foo_Compartment* Foo_Compartment_new(const char* state) {
    Foo_Compartment* c = malloc(sizeof(Foo_Compartment));
    c->state = state;
    c->state_args = Foo_FrameDict_new();
    c->state_vars = Foo_FrameDict_new();
    c->enter_args = Foo_FrameDict_new();
    c->exit_args = Foo_FrameDict_new();
    c->forward_event = NULL;
    c->parent_compartment = NULL;
    return c;
}

static Foo_Compartment* Foo_Compartment_copy(Foo_Compartment* src) {
    Foo_Compartment* c = malloc(sizeof(Foo_Compartment));
    c->state = src->state;
    c->state_args = Foo_FrameDict_copy(src->state_args);
    c->state_vars = Foo_FrameDict_copy(src->state_vars);
    c->enter_args = Foo_FrameDict_copy(src->enter_args);
    c->exit_args = Foo_FrameDict_copy(src->exit_args);
    c->forward_event = src->forward_event;  // Shallow copy OK
    c->parent_compartment = src->parent_compartment;
    return c;
}

static void Foo_Compartment_destroy(Foo_Compartment* c) {
    Foo_FrameDict_destroy(c->state_args);
    Foo_FrameDict_destroy(c->state_vars);
    Foo_FrameDict_destroy(c->enter_args);
    Foo_FrameDict_destroy(c->exit_args);
    free(c);
}
```

### 3.4 System Structure

**Exactly matches PRT:**

```c
typedef struct Foo {
    // Runtime infrastructure
    Foo_Compartment* __compartment;           // Current state
    Foo_Compartment* __next_compartment;      // Deferred transition target
    Foo_FrameVec* _state_stack;               // Stack of compartments
    Foo_FrameVec* _context_stack;             // Stack of Foo_FrameContext*

    // Domain variables (system-specific)
    int max_retries;
    char* label;
    // ... etc
} Foo;
```

---

## 4. Feature-by-Feature Implementation

### 4.1 Source File Structure

| Frame Syntax | C Implementation |
|--------------|------------------|
| Preamble (native code) | Pass through as-is (likely `#include` statements) |
| `@@target c` | Triggers C code generation |
| `@@codegen { ... }` | Configures generation options |
| `@@persist` | Generates JSON serialize/deserialize functions |
| `@@system Name { ... }` | Generates struct + functions |
| Postamble (native code) | Pass through as-is |

**File extension:** `.fc` (Frame C) for source, generates `.h` + `.c`

### 4.2 Interface Methods

| Frame | C |
|-------|---|
| `start()` | `void MySystem_start(MySystem* self)` |
| `process(data, priority)` | `void MySystem_process(MySystem* self, void* data, void* priority)` |
| `getStatus(): str` | `char* MySystem_getStatus(MySystem* self)` |
| `compute(a: int, b: int): int` | `int MySystem_compute(MySystem* self, int a, int b)` |
| `getDecision(): str = "yes"` | Default return value in context initialization |

**Interface method pattern (exactly like PRT):**

```c
int MySystem_compute(MySystem* self, int a, int b) {
    // Create event with parameters
    FrameDict* params = FrameDict_new();
    FrameDict_set(params, "a", (void*)(intptr_t)a);
    FrameDict_set(params, "b", (void*)(intptr_t)b);
    FrameEvent* __e = FrameEvent_new("compute", params);

    // Create and push context
    FrameContext* __ctx = FrameContext_new(__e, NULL);
    FrameVec_push(self->_context_stack, __ctx);

    // Route through kernel
    MySystem_kernel(self, __e);

    // Pop context and return
    FrameContext* ctx = FrameVec_pop(self->_context_stack);
    int result = (int)(intptr_t)ctx->_return;

    // Cleanup
    FrameContext_destroy(ctx);
    FrameDict_destroy(params);
    FrameEvent_destroy(__e);

    return result;
}
```

### 4.3 Machine Section - States

| Frame | C |
|-------|---|
| `$Ready { ... }` | `static void MySystem_state_Ready(MySystem* self, FrameEvent* __e)` |
| `$Processing => $Base { ... }` | Same + parent tracking |
| First state = start state | `Compartment_new("Ready")` in constructor |

### 4.4 Event Handlers

| Frame | C |
|-------|---|
| `process(data) { ... }` | `if (strcmp(__e->_message, "process") == 0) { ... }` |
| `$>() { ... }` | `if (strcmp(__e->_message, "$>") == 0) { ... }` |
| `<$() { ... }` | `if (strcmp(__e->_message, "<$") == 0) { ... }` |
| `$>(reason) { ... }` | Access via `FrameDict_get(__e->_parameters, "reason")` |

**State handler pattern:**

```c
static void MySystem_state_Ready(MySystem* self, FrameEvent* __e) {
    if (strcmp(__e->_message, "$>") == 0) {
        // State variable initialization
        FrameDict_set(self->__compartment->state_vars, "counter", (void*)0);
        // Enter handler body...
    }
    else if (strcmp(__e->_message, "<$") == 0) {
        // Exit handler body...
    }
    else if (strcmp(__e->_message, "process") == 0) {
        void* data = FrameDict_get(__e->_parameters, "data");
        // Handler body...
    }
    // Unhandled events: do nothing (explicit-only forwarding)
}
```

### 4.5 State Variables (`$.varName`)

| Frame | C Expansion |
|-------|-------------|
| `$.counter = 5` | `FrameDict_set(self->__compartment->state_vars, "counter", (void*)(intptr_t)5)` |
| `$.counter` (read) | `(int)(intptr_t)FrameDict_get(self->__compartment->state_vars, "counter")` |
| `$.label: str = "default"` | `FrameDict_set(..., "label", strdup("default"))` |
| `$.data = {}` | `FrameDict_set(..., "data", FrameDict_new())` |

### 4.6 Transitions

| Frame | C Expansion |
|-------|-------------|
| `-> $Target` | Create compartment, call transition, return |
| `-> $Target(a, b)` | Set state_args before transition |
| `-> (x, y) $Target` | Set enter_args before transition |
| `(reason) -> $Target` | Set exit_args on current compartment |
| `-> => $Target` | Set forward_event before transition |
| `-> pop$` | Pop from state stack, transition to it |

**Simple transition:**

```c
// -> $Processing
{
    Compartment* __compartment = Compartment_new("Processing");
    MySystem_transition(self, __compartment);
    return;
}
```

**Transition with all args:**

```c
// (cleanup_reason) -> (init_data) $Target(config)
{
    // Exit args on current compartment
    FrameDict_set(self->__compartment->exit_args, "0", cleanup_reason);

    // Create target compartment
    Compartment* __compartment = Compartment_new("Target");

    // State args
    FrameDict_set(__compartment->state_args, "0", config);

    // Enter args
    FrameDict_set(__compartment->enter_args, "0", init_data);

    MySystem_transition(self, __compartment);
    return;
}
```

**Event forwarding:**

```c
// -> => $Target
{
    Compartment* __compartment = Compartment_new("Target");
    __compartment->forward_event = __e;  // Stash event
    MySystem_transition(self, __compartment);
    return;
}
```

### 4.7 HSM Parent Forward (`=> $^`)

| Frame | C |
|-------|---|
| `=> $^` (in handler) | `MySystem_state_Parent(self, __e); return;` |
| `=> $^` (state-level default) | `else { MySystem_state_Parent(self, __e); }` |

**Parent tracking:**

```c
// $Child => $Parent
static void MySystem_state_Child(MySystem* self, FrameEvent* __e) {
    if (strcmp(__e->_message, "handled_event") == 0) {
        // Handle locally
    }
    else if (strcmp(__e->_message, "partial") == 0) {
        // Do something
        MySystem_state_Parent(self, __e);  // => $^
        return;
    }
    // With state-level => $^:
    else {
        MySystem_state_Parent(self, __e);
    }
}
```

### 4.8 State Stack (`push$` / `pop$`)

| Frame | C |
|-------|---|
| `push$` | `FrameVec_push(self->_state_stack, Compartment_copy(self->__compartment))` |
| `pop$` | `Compartment_destroy(FrameVec_pop(self->_state_stack))` |
| `-> pop$` | `MySystem_transition(self, FrameVec_pop(self->_state_stack))` |

### 4.9 System Context (`@@`)

| Frame | C Expansion |
|-------|-------------|
| `@@.x` | `FrameDict_get(((FrameContext*)FrameVec_last(self->_context_stack))->event->_parameters, "x")` |
| `@@:params[x]` | Same as above |
| `@@:return = value` | `((FrameContext*)FrameVec_last(self->_context_stack))->_return = value` |
| `@@:return` (read) | `((FrameContext*)FrameVec_last(self->_context_stack))->_return` |
| `@@:event` | `((FrameContext*)FrameVec_last(self->_context_stack))->event->_message` |
| `@@:data[key]` | `FrameDict_get(((FrameContext*)FrameVec_last(self->_context_stack))->_data, "key")` |
| `@@:data[key] = v` | `FrameDict_set(((FrameContext*)FrameVec_last(self->_context_stack))->_data, "key", v)` |

**Macro helpers for readability:**

```c
#define FRAME_CTX(self) ((FrameContext*)FrameVec_last((self)->_context_stack))
#define FRAME_PARAM(self, key) FrameDict_get(FRAME_CTX(self)->event->_parameters, key)
#define FRAME_RETURN(self) FRAME_CTX(self)->_return
#define FRAME_DATA(self, key) FrameDict_get(FRAME_CTX(self)->_data, key)
#define FRAME_DATA_SET(self, key, val) FrameDict_set(FRAME_CTX(self)->_data, key, val)
```

### 4.10 Return Sugar

| Frame (in handler) | C |
|--------------------|---|
| `return expr` | `FRAME_RETURN(self) = expr; return;` |
| `return` (bare) | `return;` (native) |

| Frame (in action) | C |
|-------------------|---|
| `return expr` | `return expr;` (native function return) |

### 4.11 Actions

| Frame | C |
|-------|---|
| `actions: validate() { ... }` | `static bool MySystem_validate(MySystem* self) { ... }` |

Actions CAN access:
- Domain vars: `self->domain_var`
- State vars: `FrameDict_get(self->__compartment->state_vars, "x")`
- Context: `FRAME_RETURN(self) = ...`

Actions CANNOT:
- Call `MySystem_transition()`
- Use `push$` / `pop$`

### 4.12 Operations

| Frame | C |
|-------|---|
| `operations: utility(): int { ... }` | `int MySystem_utility(MySystem* self) { ... }` |
| `static add(a, b): int { ... }` | `int MySystem_add(int a, int b) { ... }` (no self) |

Operations:
- CAN access domain vars
- CANNOT access state vars, context, transitions

### 4.13 Domain Variables

| Frame | C |
|-------|---|
| `var count: int = 0` | `int count;` field + `self->count = 0;` in constructor |
| `var label: str = "default"` | `char* label;` field + `self->label = strdup("default");` |
| `var cache = {}` | `FrameDict* cache;` field + `self->cache = FrameDict_new();` |

### 4.14 Persistence (`@@persist`)

Generates:
- `char* MySystem_save_state(MySystem* self)` - returns JSON string
- `MySystem* MySystem_restore_state(const char* json)` - returns new instance

**JSON library:** Use cJSON (MIT license, single header) or generate minimal JSON manually.

```c
char* MySystem_save_state(MySystem* self) {
    // Serialize:
    // - __compartment (state, state_vars, state_args)
    // - _state_stack
    // - domain variables
    // Returns JSON string (caller must free)
}

MySystem* MySystem_restore_state(const char* json) {
    MySystem* self = malloc(sizeof(MySystem));
    // Parse JSON
    // Set __compartment directly (NO enter event)
    // Restore domain vars
    // Restore _state_stack
    return self;
}
```

---

## 5. Runtime Methods

### 5.1 Kernel (exactly like PRT)

```c
static void MySystem_kernel(MySystem* self, FrameEvent* __e) {
    // Step 1: Route event
    MySystem_router(self, __e);

    // Step 2: Process deferred transitions
    while (self->__next_compartment != NULL) {
        Compartment* next = self->__next_compartment;
        self->__next_compartment = NULL;

        // Exit current state
        FrameEvent exit_event = { "<$", self->__compartment->exit_args };
        MySystem_router(self, &exit_event);

        // Switch compartment
        Compartment_destroy(self->__compartment);
        self->__compartment = next;

        // Enter or forward
        if (next->forward_event == NULL) {
            FrameEvent enter_event = { "$>", self->__compartment->enter_args };
            MySystem_router(self, &enter_event);
        } else {
            FrameEvent* fwd = next->forward_event;
            next->forward_event = NULL;

            if (strcmp(fwd->_message, "$>") == 0) {
                MySystem_router(self, fwd);
            } else {
                FrameEvent enter_event = { "$>", self->__compartment->enter_args };
                MySystem_router(self, &enter_event);
                MySystem_router(self, fwd);
            }
        }
    }
}
```

### 5.2 Router

```c
static void MySystem_router(MySystem* self, FrameEvent* __e) {
    const char* state = self->__compartment->state;

    if (strcmp(state, "Ready") == 0) {
        MySystem_state_Ready(self, __e);
    }
    else if (strcmp(state, "Processing") == 0) {
        MySystem_state_Processing(self, __e);
    }
    else if (strcmp(state, "Done") == 0) {
        MySystem_state_Done(self, __e);
    }
}
```

### 5.3 Transition

```c
static void MySystem_transition(MySystem* self, Compartment* next) {
    self->__next_compartment = next;
}
```

---

## 6. Constructor / Destructor

### 6.1 Constructor

```c
MySystem* MySystem_new(/* system params */) {
    MySystem* self = malloc(sizeof(MySystem));

    // Initialize stacks
    self->_state_stack = FrameVec_new();
    self->_context_stack = FrameVec_new();

    // Initialize compartment (start state)
    self->__compartment = Compartment_new("Ready");
    self->__next_compartment = NULL;

    // Initialize domain vars
    self->max_retries = 3;
    self->label = strdup("default");

    // Send initial enter event (NO context push - not an interface call)
    FrameEvent enter_event = { "$>", NULL };
    MySystem_kernel(self, &enter_event);

    return self;
}
```

### 6.2 Destructor

```c
void MySystem_destroy(MySystem* self) {
    // Destroy compartment
    Compartment_destroy(self->__compartment);

    // Destroy state stack contents
    while (FrameVec_size(self->_state_stack) > 0) {
        Compartment_destroy(FrameVec_pop(self->_state_stack));
    }
    FrameVec_destroy(self->_state_stack);

    // Destroy context stack contents (should be empty)
    while (FrameVec_size(self->_context_stack) > 0) {
        FrameContext_destroy(FrameVec_pop(self->_context_stack));
    }
    FrameVec_destroy(self->_context_stack);

    // Destroy domain vars
    free(self->label);

    free(self);
}
```

---

## 7. Test Infrastructure

### 7.1 Directory Structure

Each test is a single self-contained `.c` file (like PRT's single-file tests):

```
framepiler_test_env/
├── c_test_crate/
│   ├── Makefile
│   ├── cJSON.h              # JSON library (for persistence tests only)
│   ├── cJSON.c
│   ├── bin/                 # Compiled executables
│   └── tests/
│       ├── 01_minimal.c     # Self-contained: runtime + system + main()
│       ├── 02_interface.c
│       ├── 03_transition.c
│       └── ...
```

### 7.2 Single-File Test Pattern

Each generated `.c` file contains everything:

```c
// 01_minimal.c - SELF-CONTAINED

#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <assert.h>

// ============================================================================
// MinimalTest Runtime (generated)
// ============================================================================

typedef struct MinimalTest_FrameDictEntry { ... } MinimalTest_FrameDictEntry;
typedef struct MinimalTest_FrameDict { ... } MinimalTest_FrameDict;
// ... all runtime types and functions with MinimalTest_ prefix ...

// ============================================================================
// MinimalTest System (generated)
// ============================================================================

typedef struct MinimalTest { ... } MinimalTest;
// ... kernel, router, states, interface methods ...

// ============================================================================
// Test Harness (from Frame source postamble)
// ============================================================================

int main(void) {
    printf("=== Test 01: Minimal ===\n");

    MinimalTest* s = MinimalTest_new();
    assert(s != NULL);
    assert(strcmp(s->__compartment->state, "Ready") == 0);

    MinimalTest_destroy(s);

    printf("PASS\n");
    return 0;
}
```

### 7.3 Makefile

```makefile
CC = gcc
CFLAGS = -Wall -Wextra -Werror -g -std=c11

# Simple tests (no JSON dependency)
SIMPLE_TESTS = $(filter-out tests/23_% tests/24_% tests/25_%,$(wildcard tests/*.c))
SIMPLE_BINS = $(patsubst tests/%.c,bin/%,$(SIMPLE_TESTS))

# Persistence tests (need cJSON)
PERSIST_TESTS = tests/23_persist_basic.c tests/24_persist_roundtrip.c tests/25_persist_stack.c
PERSIST_BINS = $(patsubst tests/%.c,bin/%,$(PERSIST_TESTS))

all: $(SIMPLE_BINS) $(PERSIST_BINS)

# Simple tests: single file compilation
bin/%: tests/%.c
	@mkdir -p bin
	$(CC) $(CFLAGS) -o $@ $<

# Persistence tests: link with cJSON
bin/23_%: tests/23_%.c cJSON.c
	@mkdir -p bin
	$(CC) $(CFLAGS) -o $@ $< cJSON.c

bin/24_%: tests/24_%.c cJSON.c
	@mkdir -p bin
	$(CC) $(CFLAGS) -o $@ $< cJSON.c

bin/25_%: tests/25_%.c cJSON.c
	@mkdir -p bin
	$(CC) $(CFLAGS) -o $@ $< cJSON.c

test: $(SIMPLE_BINS) $(PERSIST_BINS)
	@for bin in bin/*; do echo "=== Running $$bin ==="; ./$$bin || exit 1; done

valgrind: $(SIMPLE_BINS) $(PERSIST_BINS)
	@for bin in bin/*; do valgrind --leak-check=full --error-exitcode=1 ./$$bin || exit 1; done

clean:
	rm -rf bin
```

---

## 8. Implementation Phases

### Phase 1: Minimal System (Tests 01-03)
- [ ] Per-system runtime generation (FrameDict, FrameVec, etc.)
- [ ] Basic struct generation
- [ ] Constructor / destructor
- [ ] Kernel / router / transition
- [ ] Simple state dispatch
- [ ] Basic transitions

### Phase 2: Native Code & Enter/Exit (Tests 04-06)
- [ ] Preamble / postamble pass-through
- [ ] Enter / exit handlers
- [ ] Domain variables

### Phase 3: Parameters (Test 07)
- [ ] Event parameters via FrameDict
- [ ] Parameter access in handlers

### Phase 4: HSM (Test 08)
- [ ] Parent state tracking
- [ ] `=> $^` in-handler forward
- [ ] State-level default forward

### Phase 5: State Stack (Test 09)
- [ ] `push$` with Compartment_copy
- [ ] `pop$` standalone
- [ ] `-> pop$` transition

### Phase 6: State Variables (Tests 10-12)
- [ ] State var declarations
- [ ] Initialization on enter
- [ ] Preservation on push/pop

### Phase 7: System Context (Tests 13-16, 36-38)
- [ ] FrameContext implementation
- [ ] Context stack push/pop
- [ ] `@@.param`, `@@:return`, `@@:event`, `@@:data`
- [ ] Reentrancy tests

### Phase 8: Extended Transitions (Tests 17-20)
- [ ] Enter args
- [ ] Exit args
- [ ] Event forwarding
- [ ] Pop transition

### Phase 9: Actions & Operations (Tests 21-22)
- [ ] Action functions (static, private)
- [ ] Operation functions (public)
- [ ] Static operations (no self)

### Phase 10: Persistence (Tests 23-25)
- [ ] cJSON integration (linked, not generated)
- [ ] `save_state()` implementation
- [ ] `restore_state()` implementation

### Phase 11: Remaining Features (Tests 26-38)
- [ ] State parameters
- [ ] Forward enter first
- [ ] HSM default forward
- [ ] Document history
- [ ] Context tests (36-38)

---

## 9. Code Generation Changes

### Files to Modify

| File | Changes |
|------|---------|
| `framec/src/frame_c/visitors/mod.rs` | Add `TargetLanguage::C` if not present |
| `framec/src/frame_c/v4/codegen/backends/c.rs` | Complete rewrite for V4 parity |
| `framec/src/frame_c/v4/codegen/system_codegen.rs` | C-specific codegen paths |
| `framec/src/frame_c/v4/pipeline/compiler.rs` | Register C target |

### Backend Implementation

The C backend emits a **single self-contained `.c` file** containing:

1. Standard includes (`stdlib.h`, `string.h`, `stdio.h`, `stdbool.h`)
2. Per-system runtime types and functions (all `static` except public API):
   - `SystemName_FrameDict` and helpers
   - `SystemName_FrameVec` and helpers
   - `SystemName_FrameEvent`
   - `SystemName_FrameContext`
   - `SystemName_Compartment`
3. System struct definition
4. Static kernel/router/transition functions
5. Static state handler functions
6. Public interface methods
7. Public constructor/destructor
8. Native preamble/postamble (including `main()` for tests)

**Future option:** `@@codegen { c_header: separate }` could generate `.h` + `.c` pair.

---

## 10. Success Criteria

- [ ] **29/29 tests passing** (parity with PRT)
- [ ] **No memory leaks** (Valgrind clean on all tests)
- [ ] **Compiles clean** with `-Wall -Wextra -Werror` on gcc and clang
- [ ] **Full feature parity** - every Frame construct works identically to PRT
- [ ] **Runtime library tested** independently

---

## 11. Non-Goals (Out of Scope)

- Thread safety (same as PRT - not thread-safe by default)
- Optimization beyond correctness
- C89 compatibility (we use C11)
- Windows-specific code (POSIX assumed for now)

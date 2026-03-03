# Adding a New Language Backend to Frame V4

This guide documents the phased process for adding a new target language backend to the Frame V4 transpiler. Each phase has specific tests that must pass before proceeding to the next phase.

## Overview

Frame V4 uses a multi-stage code generation pipeline:

```
Source (.frm/.fpy/.fts/.fc)
    → PrologScanner (find @@target)
    → PragmaScanner (separate pragmas from native code)
    → Parser (frame_parser.rs)
    → AST (frame_ast.rs)
    → Arcanum (symbol table)
    → NativeRegionScanner (find Frame statements in bodies)
    → CodegenNode (intermediate representation)
    → Language Backend (emit target code)
```

---

## Test Infrastructure

Tests are located in `framepiler_test_env/tests/common/` and organized into categories:

| Category | Tests | Purpose |
|----------|-------|---------|
| `primary/` | 36 | Core feature progression (numbered 01-38) |
| `automata/` | 2 | Moore/Mealy machine patterns |
| `capabilities/` | 5 | Specific capabilities validation |
| `control_flow/` | 39 | Frame statements in control flow contexts |
| `core/` | 20 | Core functionality (CLI, snapshots) |
| `data_types/` | 6 | Data type handling (dict, list, strings) |
| `exec_smoke/` | 5 | Quick execution smoke tests |
| `interfaces/` | 2 | Interface-specific tests |
| `operators/` | 5 | Arithmetic, comparison, logical operators |
| `scoping/` | 3 | Variable scoping, shadowing |
| `systems/` | 11 | System-level tests |
| `validator/` | 3 | Terminal statement validation |

**Total: ~137 tests**

File extensions: `.fpy` (Python), `.fts` (TypeScript), `.frs` (Rust), `.fc` (C)

---

## Prerequisite: Scanner Infrastructure

Before implementing code generation, you must implement the language-specific scanning components. These scanners use state machines to properly identify Frame constructs while respecting your language's string and comment syntax.

### Required Scanner Components

| Component | File Location | Purpose |
|-----------|---------------|---------|
| `SyntaxSkipper` | `native_region_scanner/<lang>.rs` | Skip strings/comments for your language |
| `BodyCloser` | `body_closer/<lang>.rs` | Find matching `}` respecting strings/comments |
| `NativeRegionScanner` | `native_region_scanner/<lang>.rs` | Find Frame statements in handler bodies |

### Step 0.1: Implement SyntaxSkipper

The `SyntaxSkipper` trait tells scanners how to skip your language's protected regions (strings and comments). This is **critical** for the "oceans model" - native code must pass through unchanged, and Frame constructs inside strings/comments must be ignored.

**File:** `framec/src/frame_c/v4/native_region_scanner/yourlang.rs`

```rust
use super::*;
use super::unified::*;
use crate::frame_c::v4::body_closer::yourlang::BodyCloserYourLang;
use crate::frame_c::v4::body_closer::BodyCloser;

pub struct NativeRegionScannerYourLang;

/// YourLang syntax skipper - handles comments and strings
struct YourLangSkipper;

impl SyntaxSkipper for YourLangSkipper {
    fn body_closer(&self) -> Box<dyn BodyCloser> {
        Box::new(BodyCloserYourLang)
    }

    fn skip_comment(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
        // Implement for your language's comment syntax
        // Return Some(position_after_comment) if comment found
        // Return None if no comment at position i

        // Example for // and /* */ comments:
        if let Some(j) = skip_line_comment(bytes, i, end) {
            return Some(j);
        }
        skip_block_comment(bytes, i, end)
    }

    fn skip_string(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
        // Implement for your language's string syntax
        // Handle escape sequences, multiline strings, etc.

        // Example for simple "..." strings:
        skip_simple_string(bytes, i, end)
    }

    fn find_line_end(&self, bytes: &[u8], start: usize, end: usize) -> usize {
        // Find end of Frame statement line
        // Usually: newline, semicolon, or comment start
        find_line_end_c_like(bytes, start, end)  // Works for C-family languages
    }

    fn balanced_paren_end(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
        // Find matching ) for (, respecting strings
        balanced_paren_end_c_like(bytes, i, end)
    }
}

impl NativeRegionScanner for NativeRegionScannerYourLang {
    fn scan(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<ScanResult, ScanError> {
        scan_native_regions(&YourLangSkipper, bytes, open_brace_index)
    }
}
```

### Step 0.2: Implement BodyCloser

The `BodyCloser` finds the matching `}` for a `{`, respecting strings and comments.

**File:** `framec/src/frame_c/v4/body_closer/yourlang.rs`

```rust
use super::*;

pub struct BodyCloserYourLang;

impl BodyCloser for BodyCloserYourLang {
    fn close_byte(&self, bytes: &[u8], open: usize) -> Result<usize, CloseError> {
        // Scan forward from open, tracking brace depth
        // Skip strings and comments
        // Return position of matching }

        // Use shared implementation for C-family languages:
        close_byte_c_family(bytes, open)
    }
}
```

### Step 0.3: Register Scanner

**File:** `framec/src/frame_c/v4/native_region_scanner/mod.rs`

```rust
pub mod yourlang;
```

**File:** `framec/src/frame_c/v4/body_closer/mod.rs`

```rust
pub mod yourlang;
```

### Unified Helper Functions

The `unified.rs` module provides shared helper functions:

| Function | Purpose |
|----------|---------|
| `skip_line_comment(bytes, i, end)` | Skip `// ...` to newline |
| `skip_block_comment(bytes, i, end)` | Skip `/* ... */` |
| `skip_simple_string(bytes, i, end)` | Skip `"..."` with backslash escapes |
| `find_line_end_c_like(bytes, start, end)` | Find line end for C-family languages |
| `balanced_paren_end_c_like(bytes, i, end)` | Find matching `)` for C-family |
| `scan_native_regions(skipper, bytes, open)` | Main scanning loop (unified) |

### Why This Matters

The `PragmaScanner` and `NativeRegionScanner` use your `SyntaxSkipper` to ensure:

1. **`@@` inside strings is NOT a pragma:**
   ```python
   print("@@target python_3")  # Not a pragma
   ```

2. **Frame statements inside comments are ignored:**
   ```javascript
   // -> $NextState  // Not a transition
   ```

3. **Native code passes through verbatim:**
   ```rust
   let s = "-> $State";  // String content preserved exactly
   ```

Without correct `SyntaxSkipper` implementation, the transpiler will incorrectly parse native code or miss Frame constructs.

---

## Implementation Phases

The implementation is divided into 6 phases. **Primary tests** validate core functionality per phase. **Extended tests** provide comprehensive coverage.

| Phase | Feature Set | Primary Tests | Extended Tests |
|-------|-------------|---------------|----------------|
| 1 | Core System Structure | 01, 02, 04, 06 | core/*, interfaces/* |
| 2 | Transitions & Lifecycle | 03, 05 | control_flow/transition_*, automata/* |
| 3 | Parameters & Return Values | 07, 13-16, 35 | capabilities/system_return_*, interfaces/* |
| 4 | State Variables & Stack | 09-12, 20 | core/stack_*, exec_smoke/stack_* |
| 5 | Hierarchical State Machines | 08, 19, 29, 30 | control_flow/forward_*, systems/parent_* |
| 6 | Actions, Operations & Persistence | 21-26 | capabilities/actions_*, capabilities/operations_* |

---

## Phase 1: Core System Structure

**Goal:** Generate a compilable system class with basic interface methods and event routing.

### Features to Implement
- System class generation
- Constructor with initial state
- Interface method wrappers (public API)
- `__kernel` method (event dispatcher)
- `__router` method (state dispatch)
- State handler methods (one per state)
- `__compartment` field (current state closure)
- Runtime types: `FrameEvent`, `Compartment`

### Primary Validation Tests

| Test | Description | Key Features |
|------|-------------|--------------|
| `primary/01_minimal` | Single state, single method | System instantiation, interface method, return value |
| `primary/02_interface` | Multiple interface methods | Multiple handlers, method signatures |
| `primary/04_native_code` | Native code pass-through | "Oceans model" - native code preserved |
| `primary/06_domain_vars` | Domain variables | `domain:` section, field initialization |

### Extended Validation Tests

| Category | Tests | Validates |
|----------|-------|-----------|
| `core/simple_interface` | 1 | Basic interface generation |
| `core/basic_cli_compile` | 1 | CLI compilation |
| `core/basic_project` | 1 | Project structure |
| `interfaces/interface_handlers_emitted` | 1 | Handler emission |
| `interfaces/interface_with_param` | 1 | Parameterized interfaces |
| `data_types/*` | 6 | Dict, list, string, int handling |
| `operators/*` | 5 | Arithmetic, comparison, logical, ternary |

### Implementation Steps

#### 1.1 Add Language Variant

**File:** `framec/src/frame_c/visitors/mod.rs`

```rust
pub enum TargetLanguage {
    Python3,
    TypeScript,
    Rust,
    C,
    YourLang,    // Add your language
}
```

#### 1.2 Create Backend File

**File:** `framec/src/frame_c/v4/codegen/backends/yourlang.rs`

```rust
use crate::frame_c::visitors::TargetLanguage;
use crate::frame_c::v4::codegen::ast::*;
use crate::frame_c::v4::codegen::backend::*;

pub struct YourLangBackend;

impl LanguageBackend for YourLangBackend {
    fn emit(&self, node: &CodegenNode, ctx: &mut EmitContext) -> String {
        match node {
            CodegenNode::Class { name, fields, methods, .. } => {
                // Generate class/struct definition
            }
            CodegenNode::Method { name, params, return_type, body, .. } => {
                // Generate method/function
            }
            CodegenNode::Constructor { params, body, .. } => {
                // Generate constructor
            }
            // Handle all CodegenNode variants...
            _ => String::new(),
        }
    }

    fn runtime_imports(&self) -> Vec<String> { vec![] }
    fn class_syntax(&self) -> ClassSyntax { ClassSyntax::yourlang() }
    fn target_language(&self) -> TargetLanguage { TargetLanguage::YourLang }
    fn null_keyword(&self) -> &'static str { "null" }
    fn true_keyword(&self) -> &'static str { "true" }
    fn false_keyword(&self) -> &'static str { "false" }
}
```

#### 1.3 Register Backend

**File:** `framec/src/frame_c/v4/codegen/backends/mod.rs`

```rust
pub mod yourlang;
pub use yourlang::YourLangBackend;
```

**File:** `framec/src/frame_c/v4/codegen/backend.rs`

```rust
pub fn get_backend(lang: TargetLanguage) -> Box<dyn LanguageBackend> {
    match lang {
        TargetLanguage::YourLang => Box::new(backends::YourLangBackend),
        // ...
    }
}

impl ClassSyntax {
    pub fn yourlang() -> Self {
        ClassSyntax {
            language: TargetLanguage::YourLang,
            class_keyword: "class",
            self_keyword: "this",
            field_prefix: "",
            method_prefix: "",
            static_keyword: "static",
            visibility_prefix: true,
        }
    }
}
```

#### 1.4 Generate Runtime Types

**File:** `framec/src/frame_c/v4/codegen/system_codegen.rs`

Generate per-system runtime types:
- `{System}_FrameEvent` - Event object with `_message` and `_parameters`
- `{System}_Compartment` - State closure with 6 fields:
  - `state` - State handler function/method reference
  - `state_args` - Arguments to the state (for parameterized states)
  - `state_vars` - State-local variables
  - `enter_args` - Arguments passed on enter (`-> (args) $State`)
  - `exit_args` - Arguments passed on exit (`-> $State (args)`)
  - `forward_event` - Event to forward to parent (HSM)

#### 1.5 Generate Frame Machinery

Add cases in `generate_frame_machinery()` for:
- `__kernel` - Routes event to current state, processes deferred transitions
- `__router` - Dispatches event to state handler based on message
- `__transition` - Caches next compartment for deferred transition

#### 1.6 Generate State Handlers

In `generate_state_method()`, generate state dispatch code that:
1. Checks `__e._message` against event names
2. Unpacks parameters from `__e._parameters`
3. Executes handler body

### Phase 1 Verification

```bash
# Build
cargo build --release

# Primary tests
for test in 01 02 04 06; do
    ./target/release/framec -l yourlang primary/${test}_*.yourlang -o output.yourlang
    yourlang output.yourlang || echo "FAIL: primary/$test"
done

# Extended tests
for test in core/simple_interface core/basic_project; do
    ./target/release/framec -l yourlang ${test}.yourlang -o output.yourlang
    yourlang output.yourlang || echo "FAIL: $test"
done
```

---

## Phase 2: Transitions & Lifecycle

**Goal:** Implement state transitions with proper enter/exit handler invocation.

### Features to Implement
- Basic transition (`-> $State`)
- Enter handler (`$>`)
- Exit handler (`$<`)
- Deferred transition pattern (transition executes after handler completes)

### Primary Validation Tests

| Test | Description | Key Features |
|------|-------------|--------------|
| `primary/03_transition` | Basic state transition | `-> $State`, state tracking |
| `primary/05_enter_exit` | Enter/exit handlers | `$>()`, `<$()`, lifecycle order |

### Extended Validation Tests

| Category | Tests | Validates |
|----------|-------|-----------|
| `control_flow/transition_basic` | 1 | Basic transition syntax |
| `control_flow/transition_basic_exec` | 1 | Transition execution |
| `control_flow/transition_state_args_exec` | 1 | Transition with state args |
| `control_flow/transition_state_id_exec` | 1 | State identification |
| `automata/moore_machine` | 1 | Moore pattern (output on entry) |
| `automata/mealy_machine` | 1 | Mealy pattern (output on transition) |
| `systems/transition_basic_exec` | 1 | System-level transition |

### Implementation Steps

#### 2.1 Transition Expansion

In `generate_frame_expansion()` for `FrameSegmentKind::Transition`:

```rust
TargetLanguage::YourLang => {
    // 1. Create new compartment for target state
    // 2. Call __transition(compartment) to cache it
    // Result: let __c = new Compartment(__state_Target)
    //         __transition(__c)
}
```

#### 2.2 Kernel Transition Processing

The `__kernel` method must:
1. Route event to current state
2. Check if `__next_compartment` is set
3. If set: send `<$` (exit) to current, switch compartment, send `$>` (enter) to new

#### 2.3 Enter/Exit Handlers

State methods must recognize `$>` and `$<` as special event names:
- `$>` = enter event (sent after transition completes)
- `$<` = exit event (sent before leaving state)

### Phase 2 Verification

```bash
# Primary tests
for test in 03 05; do
    ./target/release/framec -l yourlang primary/${test}_*.yourlang -o output.yourlang
    yourlang output.yourlang || echo "FAIL: primary/$test"
done

# Extended tests - automata
for test in automata/moore_machine automata/mealy_machine; do
    ./target/release/framec -l yourlang ${test}.yourlang -o output.yourlang
    yourlang output.yourlang || echo "FAIL: $test"
done

# Extended tests - control_flow/transition_*
ls control_flow/transition_*.yourlang | while read test; do
    ./target/release/framec -l yourlang "$test" -o output.yourlang
    yourlang output.yourlang || echo "FAIL: $test"
done
```

---

## Phase 3: Parameters & Return Values

**Goal:** Support event parameters and interface return values.

### Features to Implement
- Event parameter passing via `__e._parameters`
- `return expr` sugar (sets `@@:return` and exits handler)
- `@@:return = expr` explicit assignment
- Interface header default return values (`method(): int = 10`)
- Context stack for reentrancy

### Primary Validation Tests

| Test | Description | Key Features |
|------|-------------|--------------|
| `primary/07_params` | Event parameters | Parameter unpacking from `_parameters` |
| `primary/13_system_return` | Return values | `return expr`, `@@:return = expr` |
| `primary/14_system_return_default` | Default returns | Interface header `= default` |
| `primary/15_system_return_chain` | Chained returns | Return through multiple states |
| `primary/16_system_return_reentrant` | Nested calls | Reentrant interface calls |
| `primary/35_return_init` | Return initialization | Default values in interface |

### Extended Validation Tests

| Category | Tests | Validates |
|----------|-------|-----------|
| `capabilities/system_return_header_defaults` | 1 | Interface header default values |
| `control_flow/handler_params_manifest` | 1 | Handler parameter manifest |
| `systems/interface_with_param` | 1 | Parameterized interface |
| `interfaces/interface_with_param` | 1 | Interface parameter passing |

### Implementation Steps

#### 3.1 Parameter Unpacking

In state dispatch, extract parameters:

```rust
// For handler: event(a: int, b: str)
// Generate: a = __e._parameters["a"]
//           b = __e._parameters["b"]
```

#### 3.2 Context Stack

Add `_context_stack` field and `FrameContext` type:

```rust
class FrameContext {
    event: FrameEvent      // Reference to interface event
    _return: any           // Return value slot
    _data: dict            // Call-scoped data
}
```

Interface methods:
1. Create `FrameEvent` with method name and parameters
2. Create `FrameContext` wrapping the event
3. Push context to `_context_stack`
4. Call `__kernel(__e)`
5. Pop context and return `_return` value

#### 3.3 System Return Expansion

For `FrameSegmentKind::ReturnSugar`:
```rust
// "return expr" -> set return value and exit
TargetLanguage::YourLang => {
    // _context_stack[-1]._return = expr
    // return
}
```

For `FrameSegmentKind::ContextReturnAssign`:
```rust
// "@@:return = expr" -> set return value, continue
TargetLanguage::YourLang => {
    // _context_stack[-1]._return = expr
}
```

#### 3.4 Interface Default Returns

Parse interface headers with default values:
```
method(): int = 10
```

When generating `FrameContext`, initialize `_return` with the default.

### Phase 3 Verification

```bash
# Primary tests
for test in 07 13 14 15 16 35; do
    ./target/release/framec -l yourlang primary/${test}_*.yourlang -o output.yourlang
    yourlang output.yourlang || echo "FAIL: primary/$test"
done

# Extended tests
./target/release/framec -l yourlang capabilities/system_return_header_defaults.yourlang -o output.yourlang
yourlang output.yourlang || echo "FAIL: capabilities/system_return_header_defaults"
```

---

## Phase 4: State Variables & Stack

**Goal:** Support state-local variables and state stack for push/pop transitions.

### Features to Implement
- State variables (`$.var` syntax)
- State variable storage in `__compartment.state_vars`
- State stack (`_state_stack` field)
- `push$` - Push current compartment to stack
- `-> pop$` - Pop compartment and transition to it
- State variable preservation across push/pop

### Primary Validation Tests

| Test | Description | Key Features |
|------|-------------|--------------|
| `primary/09_stack` | Push/pop transitions | `push$`, `-> pop$` |
| `primary/10_state_var_basic` | Basic state variables | `$.var` declaration and access |
| `primary/11_state_var_reentry` | State variable reset | Variables reset on re-entry |
| `primary/12_state_var_push_pop` | Variables with stack | Variables preserved on pop |
| `primary/20_transition_pop` | Pop transition details | Pop transition mechanics |

### Extended Validation Tests

| Category | Tests | Validates |
|----------|-------|-----------|
| `core/stack_ops` | 1 | Stack operations |
| `core/stack_ops_then_native` | 1 | Stack with native code |
| `core/terminal_last_stack_ops` | 1 | Terminal stack operations |
| `exec_smoke/stack_ops` | 1 | Stack smoke test |
| `exec_smoke/stack_then_transition` | 1 | Stack then transition |
| `exec_smoke/nested_stack_then_transition` | 1 | Nested stack operations |
| `control_flow/stack_then_transition_exec` | 1 | Stack control flow |
| `control_flow/stack_pop_then_transition_exec` | 1 | Pop then transition |
| `control_flow/stack_forward_then_transition_exec` | 1 | Stack with forward |

### Implementation Steps

#### 4.1 State Stack Field

Add `_state_stack` field initialized to empty list/array.

#### 4.2 State Variable Storage

State variables live in `__compartment.state_vars`:

```rust
// $.counter = 0 becomes:
// __compartment.state_vars["counter"] = 0

// Reading $.counter:
// __compartment.state_vars["counter"]
```

#### 4.3 Push Operation

For `FrameSegmentKind::StackPush`:
```rust
TargetLanguage::YourLang => {
    // _state_stack.push(__compartment.clone())
}
```

#### 4.4 Pop Transition

For `FrameSegmentKind::StackPop` (or `-> pop$`):
```rust
TargetLanguage::YourLang => {
    // __saved = _state_stack.pop()
    // __transition(__saved)
}
```

The key insight: pop restores the **entire compartment**, including state_vars.

### Phase 4 Verification

```bash
# Primary tests
for test in 09 10 11 12 20; do
    ./target/release/framec -l yourlang primary/${test}_*.yourlang -o output.yourlang
    yourlang output.yourlang || echo "FAIL: primary/$test"
done

# Extended tests - stack operations
for test in core/stack_ops exec_smoke/stack_ops exec_smoke/stack_then_transition; do
    ./target/release/framec -l yourlang ${test}.yourlang -o output.yourlang
    yourlang output.yourlang || echo "FAIL: $test"
done
```

---

## Phase 5: Hierarchical State Machines

**Goal:** Support parent states and event forwarding.

### Features to Implement
- Parent state declaration (`$Child => $Parent`)
- Event forwarding (`=> $^` or `=> $Parent`)
- Transition with forward (`-> => $State`)
- Default forward (unhandled events automatically forward)
- `forward_event` field in compartment

### Primary Validation Tests

| Test | Description | Key Features |
|------|-------------|--------------|
| `primary/08_hsm` | Basic HSM | `=> $Parent`, `=> $^` |
| `primary/19_transition_forward` | Transition then forward | `-> => $State` |
| `primary/29_forward_enter_first` | Forward with enter | Enter before forward |
| `primary/30_hsm_default_forward` | Default forwarding | Auto-forward unhandled |

### Extended Validation Tests

| Category | Tests | Validates |
|----------|-------|-----------|
| `core/forward_parent` | 1 | Basic parent forwarding |
| `core/forward_then_native_exec` | 1 | Forward with native code |
| `core/forward_multi_native` | 1 | Multiple forwards with native |
| `control_flow/forward_*` | 15+ | Various forward patterns |
| `control_flow/child_forwards_then_transition_exec` | 1 | Child-to-parent forwarding |
| `control_flow/forwards_twice_then_transition_exec` | 1 | Multiple forwards |
| `systems/child_forwards_then_transition_exec` | 1 | System-level forwarding |
| `systems/nested_parent_forward_then_transition_exec` | 1 | Nested parent forwarding |
| `systems/parent_forward_then_stack_then_transition_exec` | 1 | Forward with stack |

### Implementation Steps

#### 5.1 Parent State Tracking

Parse `$Child => $Parent` syntax and store parent reference in state metadata.

#### 5.2 Forward Expansion

For `FrameSegmentKind::Forward`:
```rust
TargetLanguage::YourLang => {
    // __compartment.forward_event = __e
    // return  // Exit handler, kernel will forward
}
```

#### 5.3 Kernel Forward Processing

After handler returns, check `__compartment.forward_event`:
1. If set, look up parent state
2. Call parent state handler with the forwarded event
3. Clear `forward_event`

#### 5.4 Default Forward

For states with a parent but no handler for an event, automatically forward.

### Phase 5 Verification

```bash
# Primary tests
for test in 08 19 29 30; do
    ./target/release/framec -l yourlang primary/${test}_*.yourlang -o output.yourlang
    yourlang output.yourlang || echo "FAIL: primary/$test"
done

# Extended tests - forward patterns
ls control_flow/forward_*.yourlang | while read test; do
    ./target/release/framec -l yourlang "$test" -o output.yourlang
    yourlang output.yourlang || echo "FAIL: $test"
done

# Extended tests - HSM systems
for test in systems/child_forwards_then_transition_exec systems/nested_parent_forward_then_transition_exec; do
    ./target/release/framec -l yourlang ${test}.yourlang -o output.yourlang
    yourlang output.yourlang || echo "FAIL: $test"
done
```

---

## Phase 6: Actions, Operations & Persistence

**Goal:** Support action methods, operations, and state persistence.

### Features to Implement
- Actions section (`actions:`) - internal methods
- Action wrappers (public API for actions)
- Operations section (`operations:`) - pure functions
- State parameters (`$State(param: type)`)
- State serialization (`save_state()`, `load_state()`)

### Primary Validation Tests

| Test | Description | Key Features |
|------|-------------|--------------|
| `primary/21_actions_basic` | Action methods | `actions:` section |
| `primary/22_operations_basic` | Operation methods | `operations:` section |
| `primary/23_persist_basic` | Basic persistence | `save_state()` |
| `primary/24_persist_roundtrip` | Save and restore | `load_state()` |
| `primary/25_persist_stack` | Persist with stack | Stack serialization |
| `primary/26_state_params` | Parameterized states | `$State(a: int)` |

### Extended Validation Tests

| Category | Tests | Validates |
|----------|-------|-----------|
| `capabilities/actions_emitted` | 1 | Action method emission |
| `capabilities/actions_call_wrappers` | 1 | Action wrapper generation |
| `capabilities/operations_emitted` | 1 | Operation method emission |
| `capabilities/nested_functions` | 1 | Nested function handling |
| `scoping/function_scope` | 1 | Function scoping |
| `scoping/nested_functions` | 1 | Nested function scoping |
| `scoping/shadowing` | 1 | Variable shadowing |

### Implementation Steps

#### 6.1 Actions Generation

Actions are internal methods. Generate:
1. Private `_action_name()` implementation
2. Public `name()` wrapper (calls internal)

#### 6.2 Operations Generation

Operations are pure functions - generate as regular methods.

#### 6.3 Persistence

Generate `save_state()` and `load_state()`:
- Serialize `__compartment` and `_state_stack`
- Use JSON or language-native serialization

#### 6.4 State Parameters

For `$State(a: int, b: str)`:
- Store in `__compartment.state_args`
- Initialize when transitioning: `-> (a, b) $State`

### Phase 6 Verification

```bash
# Primary tests
for test in 21 22 23 24 25 26; do
    ./target/release/framec -l yourlang primary/${test}_*.yourlang -o output.yourlang
    yourlang output.yourlang || echo "FAIL: primary/$test"
done

# Extended tests - capabilities
for test in capabilities/actions_emitted capabilities/operations_emitted capabilities/actions_call_wrappers; do
    ./target/release/framec -l yourlang ${test}.yourlang -o output.yourlang
    yourlang output.yourlang || echo "FAIL: $test"
done

# Extended tests - scoping
for test in scoping/function_scope scoping/nested_functions scoping/shadowing; do
    ./target/release/framec -l yourlang ${test}.yourlang -o output.yourlang
    yourlang output.yourlang || echo "FAIL: $test"
done
```

---

## Advanced Tests

After completing all 6 phases, validate with documentation examples and context features:

### Documentation Examples

| Test | Description |
|------|-------------|
| `primary/31_doc_lamp_basic` | Basic lamp example from docs |
| `primary/32_doc_lamp_hsm` | HSM lamp example |
| `primary/33_doc_history_basic` | History pattern |
| `primary/34_doc_history_hsm` | HSM with history |

### Context Features

| Test | Description |
|------|-------------|
| `primary/36_context_basic` | Context access (`@@.`) |
| `primary/37_context_reentrant` | Reentrant context |
| `primary/38_context_data` | Context data (`@@:data`) |

### Transition Arguments

| Test | Description |
|------|-------------|
| `primary/17_transition_enter_args` | Enter arguments (`-> (args) $State`) |
| `primary/18_transition_exit_args` | Exit arguments (`-> $State (args)`) |

---

## Comprehensive Test Categories

### Validator Tests

Terminal statement validation - ensures Frame statements appear last in control flow:

| Test | Validates |
|------|-----------|
| `validator/terminal_last_transition` | Transitions must be terminal |
| `validator/terminal_last_forward` | Forwards must be terminal |
| `validator/terminal_last_stack_ops` | Stack ops must be terminal |

### Control Flow Tests (39 tests)

Comprehensive coverage of Frame statements in various control flow contexts:

| Pattern | Tests | Validates |
|---------|-------|-----------|
| `if_*` | 5 | Conditionals with Frame statements |
| `elif_*` | 2 | Elif chains |
| `while_*` | 4 | Loops with Frame statements |
| `forward_*` | 15 | Forward variations |
| `stack_*` | 5 | Stack operations in control flow |
| `transition_*` | 8 | Transition variations |

### Systems Tests (11 tests)

System-level integration tests:

| Test | Validates |
|------|-----------|
| `simple_interface` | Basic interface |
| `interface_with_param` | Parameterized interface |
| `multiple_handlers` | Multiple event handlers |
| `handler_outside_state` | Handler placement |
| `*_exec` tests | Runtime execution |

### Exec Smoke Tests (5 tests)

Quick validation of common patterns:

| Test | Validates |
|------|-----------|
| `stack_ops` | Basic stack |
| `stack_then_transition` | Stack + transition |
| `nested_stack_then_transition` | Nested stacks |
| `if_forward_else_transition` | Conditional patterns |
| `mixed_ops` | Mixed operations |

---

## Key Patterns

### Per-System Runtime Types

Each system gets its own prefixed types to avoid collisions:
- `MySystem_FrameDict`
- `MySystem_FrameEvent`
- `MySystem_FrameContext`
- `MySystem_Compartment`

### Compartment Structure

```
Compartment {
    state          // State handler reference
    state_args     // $State(a, b) parameters
    state_vars     // $.var storage
    enter_args     // -> (args) $State
    exit_args      // -> $State (args)
    forward_event  // Event to forward to parent
}
```

### Context Stack for Reentrancy

Interface calls push a `FrameContext` containing:
- Reference to `FrameEvent` (message + parameters)
- `_return` slot for return value
- `_data` dict for call-scoped data

This enables nested/reentrant calls.

### Deferred Transitions

Transitions are cached in `__next_compartment`, not executed immediately. The kernel processes them after the handler returns:
1. Send `<$` (exit) event to current state
2. Switch compartment
3. Send `$>` (enter) event to new state

---

## Test File Locations

```
framepiler_test_env/tests/common/
├── primary/          # Core feature tests (01-38)
├── automata/         # Moore/Mealy machines
├── capabilities/     # Specific capability tests
├── control_flow/     # Frame in control flow
├── core/             # Core functionality
├── data_types/       # Data type handling
├── exec_smoke/       # Smoke tests
├── interfaces/       # Interface tests
├── operators/        # Operator tests
├── scoping/          # Scoping tests
├── systems/          # System-level tests
└── validator/        # Validation tests
```

To create tests for your language, copy the Python tests and change:
1. `@@target python_3` → `@@target yourlang`
2. Native code sections (imports, test harness) to your language syntax

---

## Reference Files

- `framec/src/frame_c/v4/codegen/backends/python.rs` - Python backend (best reference)
- `framec/src/frame_c/v4/codegen/backends/typescript.rs` - TypeScript backend
- `framec/src/frame_c/v4/codegen/backends/rust.rs` - Rust backend
- `framec/src/frame_c/v4/codegen/backends/c.rs` - C backend
- `framec/src/frame_c/v4/codegen/system_codegen.rs` - Main codegen logic

---

## Language-Specific Implementation Notes

### Kernel Enter/Exit Args

The kernel creates lifecycle events with the compartment's args:
- Exit event: `FrameEvent("<$", compartment.exit_args)`
- Enter event: `FrameEvent("$>", compartment.enter_args)`

These args are stored with **positional string keys** ("0", "1", etc.) and handlers unpack them by position.

### C Backend Considerations

**Action Calls in Native Code:** Since native code passes through verbatim (oceans model), action calls within handler bodies must use C-style function call syntax:
```c
// Inside a Frame handler body:
$>() {
    SystemName_actionName(self);    // Correct C syntax
    // NOT: self->actionName();     // Python/TS style - invalid in C
}
```

**Transition Args:** C does not support Python-style tuples. Transition args like `("a", 42)` must be split and stored individually:
```c
// Correct: Split args and store with positional keys
FrameDict_set(compartment->enter_args, "0", (void*)"a");
FrameDict_set(compartment->enter_args, "1", (void*)(intptr_t)42);

// Wrong: Comma expression only stores last value
FrameDict_set(compartment->enter_args, "0", (void*)("a", 42));  // BUG!
```

**String vs Integer Storage:** C uses `void*` for generic storage:
- Strings: Cast directly `(void*)"string_literal"`
- Integers: Cast via intptr_t `(void*)(intptr_t)42`

**Retrieval:**
- Strings: `(char*)FrameDict_get(dict, "key")`
- Integers: `(int)(intptr_t)FrameDict_get(dict, "key")`

### Python/TypeScript Considerations

These languages use dictionaries naturally. Use `enumerate()` or `.map((v, i) => ...)` to create positional keys:
```python
compartment.enter_args = {str(i): v for i, v in enumerate((arg1, arg2))}
```

### Rust Considerations

Rust uses dedicated `_sv_` fields for state variables (not a HashMap). For push/pop:
- `__push_state` builds a state context struct from current `_sv_` fields
- `__pop_state` restores `_sv_` fields from the popped context

---

## Complete Validation Checklist

### Phase Completion

| Phase | Primary Tests | Extended Categories | Pass |
|-------|---------------|---------------------|------|
| 1 | 01, 02, 04, 06 | core/*, interfaces/*, data_types/*, operators/* | [ ] |
| 2 | 03, 05 | automata/*, control_flow/transition_* | [ ] |
| 3 | 07, 13-16, 35 | capabilities/system_return_* | [ ] |
| 4 | 09-12, 20 | core/stack_*, exec_smoke/stack_* | [ ] |
| 5 | 08, 19, 29, 30 | control_flow/forward_*, systems/*forward* | [ ] |
| 6 | 21-26 | capabilities/actions_*, capabilities/operations_*, scoping/* | [ ] |

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

When all tests pass, your language backend is complete.

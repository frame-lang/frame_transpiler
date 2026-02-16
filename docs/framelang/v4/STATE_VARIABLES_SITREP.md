# Frame V4 State Variables - Situation Report

**Date:** 2026-02-15
**Branch:** `frame_v4` (frame_transpiler), `main` (framepiler_test_env)
**Version:** 0.87.4
**Author:** Claude Opus 4.5 (AI-assisted development)

---

## Executive Summary

Phase 1 (State Variables) of the Frame V4 implementation is **complete**. All 36 PRT tests pass across Python, TypeScript, and Rust backends. The V4 compiler is now the default pipeline with no environment variable required.

**All three backends now correctly preserve state variables across push/pop operations.**

---

## Table of Contents

1. [Test Status](#test-status)
2. [Implementation Details](#implementation-details)
3. [Rust Compartment Architecture](#rust-compartment-architecture)
4. [Architecture Overview](#architecture-overview)
5. [Recent Changes](#recent-changes)
6. [Next Steps](#next-steps)
7. [Recommendations](#recommendations)

---

## Test Status

| Test Suite | Pass | Fail | Total |
|------------|------|------|-------|
| V4 PRT (Python) | 12 | 0 | 12 |
| V4 PRT (TypeScript) | 12 | 0 | 12 |
| V4 PRT (Rust) | 12 | 0 | 12 |
| **Total** | **36** | **0** | **36** |

### Test Coverage

| # | Test | Feature |
|---|------|---------|
| 01 | minimal | Basic system compilation |
| 02 | interface | Interface method generation |
| 03 | transition | State transitions (`-> $State`) |
| 04 | native_code | Native code preservation |
| 05 | enter_exit | `$>()` and `$<()` handlers |
| 06 | domain_vars | Domain variable initialization |
| 07 | params | Event and state parameters |
| 08 | hsm | Hierarchical state machines (`=> $^`) |
| 09 | stack | Push/pop state stack |
| 10 | state_var_basic | `$.varName` read/write |
| 11 | state_var_reentry | State vars reinitialize on entry |
| 12 | state_var_push_pop | State vars preserved across push/pop |

---

## Implementation Details

### State Variable Syntax

```frame
$Counter {
    $.count: int = 0

    increment(): int {
        $.count = $.count + 1
        return $.count
    }
}
```

### Code Generation by Language

| Language | Storage | Access Pattern |
|----------|---------|----------------|
| Python | `_state_context["varName"]` | Dictionary lookup |
| TypeScript | `_state_context["varName"]` | Object property |
| Rust | `_sv_varName` field | Struct field |

### Initialization

State variables are initialized in `_enter()` when transitioning into a state:

**Python:**
```python
def _enter(self):
    if self._state == "Counter":
        self._state_context["count"] = 0
```

**Rust:**
```rust
fn _enter(&mut self) {
    match self._state.as_str() {
        "Counter" => {
            self._sv_count = 0;
        }
        _ => {}
    }
}
```

### Push/Pop State Preservation

| Language | Preserved? | Mechanism |
|----------|------------|-----------|
| Python | ✅ Yes | Saves `(state, context.copy())` tuple |
| TypeScript | ✅ Yes | Saves `{state, context: {...}}` object |
| Rust | ✅ Yes | Saves `(state, Compartment)` tuple (typed enum) |

---

## Rust Compartment Architecture

Rust uses a "compartment enum" pattern for type-safe state variable preservation across push/pop operations.

### Generated Types

For a system with state variables, the compiler generates:

1. **Context structs** for each state with variables
2. **Compartment enum** with variants for each state
3. **Helper methods** `_state_stack_push()` and `_state_stack_pop()`

```rust
// Context struct for Counter state
#[derive(Clone, Default)]
struct CounterContext {
    count: i32,
}

// Context struct for Other state
#[derive(Clone, Default)]
struct OtherContext {
    other_count: i32,
}

// Compartment enum with typed variants
#[derive(Clone)]
enum StateVarPushPopCompartment {
    Counter(CounterContext),
    Other(OtherContext),
    Empty,
}

impl Default for StateVarPushPopCompartment {
    fn default() -> Self {
        StateVarPushPopCompartment::Counter(CounterContext::default())
    }
}
```

### Generated Struct

```rust
pub struct StateVarPushPop {
    _state: String,
    _state_stack: Vec<(String, StateVarPushPopCompartment)>,  // Typed!
    _state_context: HashMap<String, Box<dyn std::any::Any>>,
    _sv_count: i32,        // Direct field access in handlers
    _sv_other_count: i32,
}
```

### Push/Pop Methods

```rust
fn _state_stack_push(&mut self) {
    let compartment = match self._state.as_str() {
        "Counter" => StateVarPushPopCompartment::Counter(
            CounterContext { count: self._sv_count }
        ),
        "Other" => StateVarPushPopCompartment::Other(
            OtherContext { other_count: self._sv_other_count }
        ),
        _ => StateVarPushPopCompartment::Empty,
    };
    self._state_stack.push((self._state.clone(), compartment));
}

fn _state_stack_pop(&mut self) {
    let (saved_state, compartment) = self._state_stack.pop().unwrap();
    self._exit();
    self._state = saved_state;
    match compartment {
        StateVarPushPopCompartment::Counter(ctx) => {
            self._sv_count = ctx.count;
        }
        StateVarPushPopCompartment::Other(ctx) => {
            self._sv_other_count = ctx.other_count;
        }
        StateVarPushPopCompartment::Empty => {}
    }
}
```

### Benefits

1. **Type Safety**: All state variables have compile-time type checking
2. **No Runtime Downcasting**: Direct field access via `_sv_` fields
3. **Correct Push/Pop**: State variables are preserved using typed enum
4. **Performance**: No heap allocation for state variable access

---

## Architecture Overview

```
Source (.frm)
    │
    ▼
┌─────────────────┐
│  Frame Parser   │ ──► Frame AST (SystemAst)
└─────────────────┘
    │
    ▼
┌─────────────────┐
│    Arcanum      │ ──► Symbol Table (states, handlers, vars)
└─────────────────┘
    │
    ▼
┌─────────────────┐
│   Validator     │ ──► E400-E406 error codes
└─────────────────┘
    │
    ▼
┌─────────────────┐
│ Native Region   │ ──► Identifies Frame segments in handlers
│    Scanner      │     (Transition, Forward, StateVar, etc.)
└─────────────────┘
    │
    ▼
┌─────────────────┐
│    Splicer      │ ──► Replaces Frame segments with codegen
└─────────────────┘
    │
    ▼
┌─────────────────┐
│    Backend      │ ──► Target language code (Python/TS/Rust)
└─────────────────┘
```

### Key Files

| File | Purpose |
|------|---------|
| `frame_parser.rs` | Parses Frame source into AST |
| `frame_ast.rs` | AST node definitions including `StateVarAst` |
| `arcanum.rs` | Symbol table construction |
| `frame_validator.rs` | Semantic validation |
| `system_codegen.rs` | AST to CodegenNode transformation |
| `native_region_scanner/*.rs` | Language-specific Frame segment detection |
| `body_closer/*.rs` | Language-specific brace matching |

---

## Recent Changes

### This Session

- **Implemented Rust compartment enum pattern** for state variable preservation
- **Generated context structs** per state with state variables
- **Generated compartment enum** with typed variants
- **Generated `_state_stack_push` and `_state_stack_pop` methods**
- **Updated test 12** to expect correct preservation behavior

### Key Functions Added

- `generate_rust_compartment_types()` - Generates context structs and compartment enum
- `generate_rust_state_stack_push()` - Builds compartment from `_sv_` fields and pushes
- `generate_rust_state_stack_pop()` - Pops and restores `_sv_` fields from compartment
- Updated `HandlerContext` with `has_state_vars` flag for conditional code generation

---

## Next Steps

Per `frame_v4_plan.md`:

### Phase 2: System Return
- Implement `^` return statement for returning from system
- Track return types through handler chain

### Phase 3: Event Parameters
- Complete event parameter passing through dispatch
- Handle typed parameters in generated code

### Phase 4: Advanced Features
- Transition arguments (`(exit_args) -> (enter_args) $State`)
- State parameters (`$State(params)`)
- Action/operation integration

---

## Recommendations

1. **Rust warning suppression** - Add `#[allow(non_snake_case)]` to generated code to suppress handler name warnings

2. **Native code semicolons** - Document that Rust handlers require proper Rust syntax (semicolons on non-final statements)

3. **Continue to Phase 2** - System return (`^`) is next in the plan

---

## Appendix: Test Execution

To run the full test suite:

```bash
framepiler_test_env/common/test-frames/v4/prt/run_tests.sh
```

Expected output:
```
Total: 36 passed, 0 failed
```

To run individual tests:

```bash
# Python
./target/release/framec test.frm -l python_3 > test.py && python3 test.py

# TypeScript
./target/release/framec test.frm -l typescript > test.ts && npx ts-node test.ts

# Rust
./target/release/framec test.frm -l rust > test.rs && rustc test.rs -o test && ./test
```

---

**Status:** ✅ Phase 1 Complete | 36/36 Tests Passing | All Languages Preserve State Variables

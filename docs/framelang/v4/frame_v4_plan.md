# Frame V4 Implementation Plan

**Version:** 2.0
**Date:** February 2026
**Status:** Active Development
**Approach:** Test-Driven (PRT: Python, Rust, TypeScript)

---

## Current Status

**Test Results (2026-02-21):**
- Python: 28/28 tests passing (100%)
- TypeScript: 28/28 tests passing (100%)
- Rust: 28/28 tests passing (100%)

**Total: 84/84 tests passing (100%)**

**Milestone: Full Language Parity Achieved**
All three target languages (Python, TypeScript, Rust) now use the unified kernel/router/transition compartment architecture.

---

## Completed Phases

### Phase 0: Baseline Validation ✅
- All PRT languages passing core tests
- Basic transitions, enter/exit, domain vars

### Phase 1: Compartment Architecture ✅
- 6-field Compartment: state, state_args, state_vars, enter_args, exit_args, forward_event
- State variable storage in `compartment.state_vars`
- State stack stores entire compartments (push$/pop$)
- Tests 10-12 passing

### Phase 2: System Return ✅
- `system.return = value` sets interface return
- `return expr` sugar in handlers
- `_return_value` field for return chain
- Tests 13-16 passing

### Phase 3: Extended Transitions ✅
- Enter args: `-> (args) $State`
- Exit args: `(args) -> $State`
- Event forwarding: `-> => $State`
- Pop transition: `-> pop$`
- Tests 17-20 passing

### Phase 4: Actions & Operations ✅
- Actions: private helpers with state/domain access
- Operations: public methods bypassing state machine
- Tests 21-22 passing

### Phase 5: Persistence ✅
- `@@persist` generates save_state/restore_state
- Language-native serialization (pickle, JSON, serde)
- State stack serialization
- Tests 23-25 passing

### Phase 6: State Parameters ✅
- `-> $State(args)` passes to state_args
- Constructor `$(params)` for start state
- Test 26 passing

### Phase 7.1: State-Level Default Forward ✅
- `=> $^` at state level forwards ALL unhandled events to parent
- Adds else clause in state dispatch for child states
- Test 30 passing

### Phase 8: Forward Event Refinements ✅
- Kernel sends `$>` before non-`$>` forward events
- `-> =>` properly initializes state before forwarding
- Test 29 passing

### Phase 10: Test Infrastructure ✅
- Tests output to proper test crates (not /tmp)
- Python: `python_test_crate/tests/`
- TypeScript: `typescript_test_crate/tests/`
- Rust: `rust_test_crate/tests/`
- Rust tests run via cargo for dependency support

---

## Runtime Architecture (Implemented)

### Python & TypeScript Runtime

Full kernel/router/transition pattern:

```python
class System:
    __compartment: Compartment           # Current state
    __next_compartment: Compartment?     # Deferred transition
    _state_stack: list[Compartment]      # History stack
    _return_value: any                   # Return chain

    def __kernel(self, __e):
        self.__router(__e)
        while self.__next_compartment is not None:
            # Process deferred transition
            # Exit current, switch, enter new (or forward)

    def __router(self, __e):
        # Dynamic dispatch to _state_X method

    def __transition(self, compartment):
        self.__next_compartment = compartment  # Deferred
```

### Rust Runtime

Full kernel/router/transition pattern (same as Python/TypeScript):

```rust
impl System {
    __compartment: SystemCompartment,
    __next_compartment: Option<SystemCompartment>,
    _state_stack: Vec<(String, SystemStateContext)>,
    _return_value: Option<ReturnType>,

    fn __kernel(&mut self, __e: SystemFrameEvent) {
        self.__router(&__e);
        while self.__next_compartment.is_some() {
            let next_compartment = self.__next_compartment.take().unwrap();
            let exit_event = SystemFrameEvent::new("$<");
            self.__router(&exit_event);
            self.__compartment = next_compartment;
            // Enter or forward event handling...
        }
    }

    fn __router(&mut self, __e: &SystemFrameEvent) {
        match self.__compartment.state.as_str() {
            "StateA" => self._state_StateA(__e),
            "StateB" => self._state_StateB(__e),
            _ => {}
        }
    }

    fn __transition(&mut self, next_compartment: SystemCompartment) {
        self.__next_compartment = Some(next_compartment);  // Deferred
    }
}
```

---

## Completed Implementation Work

### Phase 9: Rust Compartment Architecture - COMPLETE ✅

**Rust now uses the SAME kernel/router/transition pattern as Python/TypeScript:**

| Feature | Python | TypeScript | Rust |
|---------|--------|------------|------|
| State vars | `compartment.state_vars["name"]` | `compartment.stateVars["name"]` | `__compartment.state_vars.get("name")` |
| Compartment | `__compartment` | `#compartment` | `__compartment` |
| Transitions | Deferred via `__next_compartment` | Deferred via `#nextCompartment` | Deferred via `__next_compartment` |
| Event routing | FrameEvent + kernel/router | FrameEvent + kernel/router | FrameEvent + kernel/router |
| HSM forwarding | `=> $^` → parent method call | `=> $^` → parent method call | `=> $^` → parent method call |
| State stack | `Vec[(String, StateContext)]` | `Array<Compartment>` | `Vec<(String, StateContext)>` |

**Key Implementation Details:**
- `SystemCompartment` struct with all 6 fields (state, state_args, state_vars, enter_args, exit_args, forward_event)
- `SystemFrameEvent` struct with message field
- `SystemStateContext` enum for typed state variable preservation
- Direct dispatch for interface methods with parameters (bypasses kernel for efficiency)
- Transition processing loop added to interface methods for deferred transition support

**Note:** Per language spec, there is no syntax to access another state's variables directly.
HSM parent state access is achieved via `=> $^` forwarding to parent handlers.

---

## Test Summary

| # | Test File | Status | Validates |
|---|-----------|--------|-----------|
| 01 | `01_minimal` | ✅ | Basic system instantiation |
| 02 | `02_interface` | ✅ | Interface method definitions |
| 03 | `03_transition` | ✅ | State transitions |
| 04 | `04_native_code` | ✅ | Native language integration |
| 05 | `05_enter_exit` | ✅ | State entry/exit handlers |
| 06 | `06_domain_vars` | ✅ | Domain variables |
| 07 | `07_params` | ✅ | Event parameters |
| 08 | `08_hsm` | ✅ | HSM explicit forward |
| 09 | `09_stack` | ✅ | State stack operations |
| 10 | `10_state_var_basic` | ✅ | State variables basics |
| 11 | `11_state_var_reentry` | ✅ | State variable reentry |
| 12 | `12_state_var_push_pop` | ✅ | State var push/pop |
| 13 | `13_system_return` | ✅ | System return values |
| 14 | `14_system_return_default` | ✅ | Default return values |
| 15 | `15_system_return_chain` | ✅ | Chained return values |
| 16 | `16_system_return_reentrant` | ✅ | Reentrant returns |
| 17 | `17_transition_enter_args` | ✅ | Enter transition args |
| 18 | `18_transition_exit_args` | ✅ | Exit transition args |
| 19 | `19_transition_forward` | ✅ | Forward transitions |
| 20 | `20_transition_pop` | ✅ | Pop transitions |
| 21 | `21_actions_basic` | ✅ | Basic actions |
| 22 | `22_operations_basic` | ✅ | Basic operations |
| 23 | `23_persist_basic` | ✅ | Basic persistence |
| 24 | `24_persist_roundtrip` | ✅ | Persistence roundtrip |
| 25 | `25_persist_stack` | ✅ | Persistence with stack |
| 26 | `26_state_params` | ✅ | State parameters |
| 29 | `29_forward_enter_first` | ✅ | Send $> before non-$> forward |
| 30 | `30_hsm_default_forward` | ✅ | State-level `=> $^` |

---

## Validation Commands

**V4 Test Runner:**
```bash
cd framepiler_test_env/common/test-frames/v4/prt
./run_tests.sh   # Runs all 28 tests for Python, TypeScript, Rust
```

**Single Language:**
```bash
# Python only
./run_tests.sh 2>&1 | grep python_3

# Check specific test output
cat framepiler_test_env/python_test_crate/tests/08_hsm.py
```

---

## Success Criteria

| Phase | Tests | Status |
|-------|-------|--------|
| 0-6 | 01-26 | ✅ 78/78 passing |
| 7.1 | 30 | ✅ 3/3 passing |
| 8 | 29 | ✅ 3/3 passing |
| 9 | All | ✅ Rust parity complete |

**Current:** 84/84 (100%) - Full language parity achieved

---

## Documentation

| Document | Status |
|----------|--------|
| [frame_v4_lang_reference.md](frame_v4_lang_reference.md) | ✅ Complete |
| [frame_v4_architecture.md](frame_v4_architecture.md) | ✅ Complete |
| [frame_v4_codegen_spec.md](frame_v4_codegen_spec.md) | ✅ Complete |
| [frame_v4_runtime.md](frame_v4_runtime.md) | ✅ NEW - Runtime specification |
| [frame_v4_error_codes.md](frame_v4_error_codes.md) | ✅ Complete |

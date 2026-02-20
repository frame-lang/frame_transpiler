# Frame V4 Implementation Plan

**Version:** 2.0
**Date:** February 2026
**Status:** Active Development
**Approach:** Test-Driven (PRT: Python, Rust, TypeScript)

---

## Current Status

**Test Results (2026-02-20):**
- Python: 29/29 tests passing (100%)
- TypeScript: 29/29 tests passing (100%)
- Rust: 29/29 tests passing (100%)

**Total: 87/87 tests passing (100%)**

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

Match-based dispatch (no dynamic dispatch):

```rust
impl System {
    _state: String,
    _state_stack: Vec<Box<dyn Any>>,

    pub fn method(&mut self) {
        match self._state.as_str() {
            "StateA" => self._s_StateA_method(),
            "StateB" => self._s_StateB_method(),
            _ => {}
        }
    }

    fn _transition(&mut self, target: &str) {
        self._exit();
        self._state = target.to_string();
        self._enter();
    }
}
```

---

## Remaining Implementation Work

### Phase 7.2: HSM Parent Access ✅

**Completed:**
- `$Child => $Parent` syntax parsing ✅
- `=> $^` generates direct parent call ✅
- State-level `=> $^` forwards unhandled events ✅
- `parent_compartment` field set when creating child state compartment ✅
- `$^.varName` parent state_vars access (Python/TypeScript) ✅
- Rust uses explicit forwarding for parent state access ✅
- Test 31 passing (all languages) ✅

**Implementation Notes:**
- Python/TypeScript: `$^.varName` → `self.__compartment.parent_compartment.state_vars["varName"]`
- Rust: Uses explicit method forwarding (`=> $^`) instead of direct parent var access

**Test Coverage:**
- `08_hsm` — Basic explicit forward ✅
- `30_hsm_default_forward` — State-level `=> $^` ✅
- `31_hsm_parent_vars` — Parent state variable access ✅

### Phase 9: Rust Compartment Architecture

**Current State:**
- Rust uses simplified match-based dispatch
- No FrameEvent/Compartment classes
- Tests passing via direct state tracking

**Should Implement:**
1. **Rust Compartment struct** with typed fields
2. **Rust FrameEvent struct** for event metadata
3. **Kernel pattern** matching Python/TypeScript architecture

**Trade-offs:**
- More code but consistent cross-language model
- Better support for complex features (forward event, parent_compartment)
- Required for full HSM and event forwarding semantics

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
| 31 | `31_hsm_parent_vars` | ✅ | Parent state variable access |

---

## Validation Commands

**V4 Test Runner:**
```bash
cd framepiler_test_env/common/test-frames/v4/prt
./run_tests.sh   # Runs all 29 tests for Python, TypeScript, Rust
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
| 7.2 | 31 | ✅ 3/3 passing |
| 9 | N/A | Architecture (Planned) |

**Current:** 87/87 (100%)

---

## Documentation

| Document | Status |
|----------|--------|
| [frame_v4_lang_reference.md](frame_v4_lang_reference.md) | ✅ Complete |
| [frame_v4_architecture.md](frame_v4_architecture.md) | ✅ Complete |
| [frame_v4_codegen_spec.md](frame_v4_codegen_spec.md) | ✅ Complete |
| [frame_v4_runtime.md](frame_v4_runtime.md) | ✅ NEW - Runtime specification |
| [frame_v4_error_codes.md](frame_v4_error_codes.md) | ✅ Complete |

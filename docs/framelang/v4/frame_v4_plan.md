# Frame V4 Implementation Plan

**Version:** 2.3
**Date:** 2026-02-27
**Status:** Nearing Completion
**Approach:** Test-Driven (PRTC: Python, Rust, TypeScript, C)

---

## Current Status

**Test Results (2026-03-02):**
- Python: 144/144 tests passing (100%)
- TypeScript: 126/126 tests passing (100%)
- Rust: 130/130 tests passing (100%)
- C: 139/139 tests passing (100%)

**Total PRTC: 539/539 tests passing (100%)**

**Known Issues:**
| Test | Language | Issue |
|------|----------|-------|
| (none) | - | - |

**Architecture Status:**
All four PRTC languages use the unified kernel/router/transition/context stack architecture with `@@` syntax support, FrameContext for reentrancy, and `@@persist` for serialization.

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

### Phase 11: System Context Architecture ✅
- FrameEvent as lean routing object (message + parameters only)
- FrameContext with event reference, _return slot, and _data dict
- `_context_stack` for reentrancy support
- `@@` syntax: `@@.param`, `@@:return`, `@@:event`, `@@:data[key]`, `@@:params[key]`
- Tests 36-38 passing (context_basic, context_reentrant, context_data)

---

### Phase 12: C Language Implementation ✅
- Full parity with PRT languages achieved
- FrameDict/FrameVec runtime library
- 135/136 tests passing (99%)
- See [frame_v4_c_implementation_plan.md](frame_v4_c_implementation_plan.md)

---

### Phase 13: System Usage Tagging ✅
- Optional `@@System()` syntax for tracking/validating system usage in native code
- Catches typos like `@@Calculater()` at transpile time
- Expands to native constructors: Python `System()`, TypeScript `new System()`, Rust `System::new()`, C `System_new()`
- Skips `@@` patterns inside comments and string literals
- Test 39 passing across all PRTC languages

---

## Runtime Architecture (Implemented)

### Python & TypeScript Runtime

Full kernel/router/transition/context pattern:

```python
class System:
    __compartment: Compartment           # Current state
    __next_compartment: Compartment?     # Deferred transition
    _state_stack: list[Compartment]      # History stack
    _context_stack: list[FrameContext]   # Interface call context stack

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

**All languages use the SAME kernel/router/transition/context pattern:**

| Feature | Python | TypeScript | Rust | C |
|---------|--------|------------|------|---|
| State vars | `compartment.state_vars["name"]` | `compartment.stateVars["name"]` | `__compartment.state_vars.get("name")` | `FrameDict_get(compartment->state_vars, "name")` |
| Compartment | `__compartment` | `#compartment` | `__compartment` | `self->__compartment` |
| Transitions | Deferred via `__next_compartment` | Deferred via `#nextCompartment` | Deferred via `__next_compartment` | Deferred via `self->__next_compartment` |
| Event routing | FrameEvent + kernel/router | FrameEvent + kernel/router | FrameEvent + kernel/router | FrameEvent + kernel/router |
| HSM forwarding | `=> $^` → parent method call | `=> $^` → parent method call | `=> $^` → parent method call | `=> $^` → parent function call |
| State stack | `list[Compartment]` | `Array<Compartment>` | `Vec<Compartment>` | `FrameVec*` of `Compartment*` |
| Context stack | `list[FrameContext]` | `Array<FrameContext>` | `Vec<FrameContext>` | `FrameVec*` of `FrameContext*` |

**Key Implementation Details:**
- `SystemCompartment` struct with all 6 fields (state, state_args, state_vars, enter_args, exit_args, forward_event)
- `SystemFrameEvent` struct with message field
- `SystemStateContext` enum for typed state variable preservation
- Direct dispatch for interface methods with parameters (bypasses kernel for efficiency)
- Transition processing loop added to interface methods for deferred transition support

**Note:** Per language spec, there is no syntax to access another state's variables directly.
HSM parent state access is achieved via `=> $^` forwarding to parent handlers.

---

## Test Status

Run the test runner for current results:

```bash
cd framepiler_test_env/tests
./run_tests.sh              # All languages
./run_tests.sh --python     # Single language
./run_tests.sh --category primary  # Single category
```

**Test file extensions:**
- `.fpy` — Python
- `.fts` — TypeScript
- `.frs` — Rust
- `.fc` — C

---

## Success Criteria

| Phase | Tests | Status |
|-------|-------|--------|
| 0-6 | 01-26 | ✅ All passing |
| 7.1 | 30 | ✅ 4/4 passing |
| 8 | 29 | ✅ 4/4 passing |
| 9 | All PRTC | ✅ Architecture complete |
| 10 | Infrastructure | ✅ Test crates working |
| 11 | 36-38 | ✅ All 4 languages passing |
| 12 | C | ✅ 139/139 (100%) |
| 13 | 39 | ✅ Tagged instantiation working |

**Current PRTC:** 539/539 (100%)
- Python: 144/144 (100%)
- TypeScript: 126/126 (100%)
- Rust: 130/130 (100%)
- C: 139/139 (100%)

---

---

## Future Roadmap

### Phase 14: Parent State Parameters ✅

**Feature:** Allow passing parameters to parent states in HSM declarations.

**Motivation:**
- Enable parameterized parent states for reusable state hierarchies
- Pass configuration or context data from child to parent
- Support "template" parent states with customizable behavior

**Current Syntax:**
```frame
$Child => $Parent {
    // Child inherits from Parent, but cannot pass data
}
```

**Proposed Syntax:**
```frame
$Child => $Parent(config_value, "mode") {
    // Parent receives parameters in its state_args
}
```

**Use Cases:**

1. **Configurable Error Handling:**
```frame
$NetworkError => $ErrorState("network", 3) {
    // ErrorState receives error_type="network", max_retries=3
}

$FileError => $ErrorState("file", 1) {
    // ErrorState receives error_type="file", max_retries=1
}
```

2. **Shared Behavior with Variations:**
```frame
$BlueButton => $Button("blue", "Click me") {
    // Button parent receives color and label
}

$RedButton => $Button("red", "Cancel") {
    // Same Button parent, different config
}
```

3. **Hierarchical Context Passing:**
```frame
$ProcessingA => $Processing(handler_a) {
    // Processing parent uses handler_a for callbacks
}
```

**Implementation Path:**
1. Extend parser to recognize `=> $Parent(args)` syntax
2. Store parent_args in StateAst alongside parent name
3. Codegen creates parent compartment with state_args populated
4. Parent state's `$>` handler receives args via `__e._parameters` or `compartment.state_args`

**Design Questions:**
- Should parent args be passed once at system init, or on every transition to child?
- How do parent args interact with child's own state_args?
- Should parent args be accessible via special syntax (e.g., `$^.config`)?

**Benefits:**
| Benefit | Description |
|---------|-------------|
| Reusable hierarchies | One parent state, many parameterized children |
| Cleaner architecture | Avoid duplicating parent states for variations |
| Configuration injection | Pass context without global state |
| Template patterns | Generic parent behaviors specialized by children |

---

### Phase 15: GraphViz/Diagram Generation (Planned) 📋

**Feature:** Port GraphViz DOT generation from V3 to V4 pipeline.

**Motivation:**
- State machine visualization for documentation
- VSCode extension integration for live preview
- Debug and architecture review support

**Current Status:**
- V3 has full GraphViz implementation (main branch)
- V4 has no diagram generation target

**Implementation Path:**
1. Add `Graphviz` variant to V4 `TargetLanguage` enum
2. Create GraphViz backend implementing `LanguageBackend` trait
3. Generate DOT format from `CodegenNode` tree
4. Support multi-system files with system separation

**Future Extensions:**
- Direct SVG generation (no external GraphViz dependency)
- PlantUML output format
- Interactive diagrams with clickable states

---

### Phase 16: Docker Test Infrastructure ✅

**Feature:** Improve Docker-based test infrastructure for reliable cross-platform testing.

**Motivation:**
- Consistent test environments across development machines
- Easy CI/CD integration
- Isolation from local system dependencies
- Reproducible builds and tests

**Current Status:**
- Basic Docker runner exists at `framepiler_test_env/framepiler/docker/`
- V4 test runner (`run_tests.sh`) uses local toolchain
- Some tests may have platform-specific behavior

**Implementation Path:**
1. Create Docker images for each target language:
   - `frame-test-python` - Python 3.x environment
   - `frame-test-typescript` - Node.js + TypeScript environment
   - `frame-test-rust` - Rust toolchain environment
   - `frame-test-c` - GCC/Clang C environment
2. Add docker-compose.yml for orchestrated testing
3. Integrate with V4 test runner (`--docker` flag)
4. Add CI/CD workflow files (GitHub Actions)
5. Volume mounts for source code and output inspection

**Benefits:**
| Benefit | Description |
|---------|-------------|
| Consistency | Same environment across all machines |
| Isolation | No interference from local packages |
| CI/CD ready | Direct integration with build pipelines |
| Debugging | Reproducible test failures |

**Future Extensions:**
- Matrix testing across multiple language versions
- Performance benchmarking containers
- Remote test execution

---

## Documentation

| Document | Status |
|----------|--------|
| [frame_v4_lang_reference.md](frame_v4_lang_reference.md) | ✅ Complete |
| [frame_v4_architecture.md](frame_v4_architecture.md) | ✅ Complete |
| [frame_v4_codegen_spec.md](frame_v4_codegen_spec.md) | ✅ Complete |
| [frame_v4_runtime.md](frame_v4_runtime.md) | ✅ Complete |
| [frame_v4_error_codes.md](frame_v4_error_codes.md) | ✅ Complete |
| [frame_v4_c_implementation_plan.md](frame_v4_c_implementation_plan.md) | ✅ NEW - C implementation plan |

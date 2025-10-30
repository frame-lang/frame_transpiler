# Frame LLVM Backend Implementation Plan

## Overview

This document outlines the comprehensive implementation plan for adding LLVM native code generation to the Frame transpiler. The LLVM backend will enable Frame systems to compile to native machine code with performance comparable to C/C++ while maintaining Frame's state machine semantics and debugging capabilities.

## Implementation Status

| Phase | Scope | Status | Validation Tests | Notes |
| --- | --- | --- | --- | --- |
| Phase 1 | Foundation & minimal IR | ✅ Complete (v0.86.22) | `basic/test_simple_system.frm`, `basic/test_domain_variables.frm` | Struct layouts, interface dispatch, action lowering, smoke suite wired into CI |
| Phase 2a | Runtime compartment ownership & actions | ✅ Complete (v0.86.22) | `basic/test_actions.frm`, `basic/test_action_returns.frm` | Kernel manages init/deinit; actions mutate domains; LLVM runtime linked in CI |
| Phase 2b | Queue plumbing (parent forwarding) | ✅ Complete (v0.86.25) | `basic/test_parent_forward.frm`, `basic/test_parent_forward_queue.frm`, `basic/test_action_locals.frm` | Forward-event setter and kernel pop helper now restore child state/compartment; queue loop validated end-to-end with parent forwarding plus domain counter updates. |
| Phase 2c | Queue plumbing (enter/exit & stack pop) | ✅ Complete (v0.86.25) | `basic/test_transition_queue_enter.frm`, `basic/test_transition_queue_exit.frm`, `basic/test_state_stack_pop.frm`, `basic/test_state_parameters_basic.frm`, `basic/test_enter_args.frm`, `basic/test_parent_forward_enter_args.frm`, `basic/test_event_parameters.frm`, `basic/test_state_stack_multi_pop.frm` | Enter/exit handlers emitted via dedicated state helpers; parent forwarding passes typed args; event parameters and multi-pop stack behaviour covered by LLVM fixtures. |
| Phase 3a | Hierarchical semantics & transitions | ⏳ Planned | `basic/test_parent_hierarchy.frm`, *(new)* `hierarchy/test_multi_level_queue.frm` | Full parent stack, transition args, stack-pop semantics |
| Phase 3b | Exceptions & advanced Frame features | ⏳ Planned | *(new)* `advanced/test_exception_flow.frm`, `advanced/test_async_queue.frm`, `basic/test_print_scalars.frm` | Exception propagation, async hooks, complex control flow, runtime print formatting helper |
| Phase 4 | Production polish & cross-platform | ⏳ Planned | macOS toolchain smoke (`scripts/run_llvm_release.sh`), *(new)* `ci/test_cli_help_llvm.sh` | macOS-focused toolchain validation, debug info, CLI UX, packaging |

The current workstream emphasises finishing Phase 2 on macOS first (clang/LLD, runtime dylib search paths) while keeping the new module layout portable for Linux and Windows bring-up.

## Strategic Goals

### **Primary Objectives**
1. **Native Performance**: 10-100x speedup over Python implementations
2. **Frame Semantic Preservation**: Full state machine debugging and introspection
3. **Cross-Platform Support**: Native compilation for all major platforms
4. **Zero-Overhead Abstractions**: Frame constructs compile to efficient native code
5. **Integration Quality**: Seamless integration with existing Frame toolchain

### **Target Use Cases**
- **Systems Programming**: Frame for operating system components, drivers
- **Embedded Systems**: Frame on microcontrollers and IoT devices
- **Game Development**: Frame for game logic with native performance
- **High-Performance Computing**: Frame for numerical algorithms and simulations
- **Real-Time Systems**: Frame for deterministic real-time applications

## Technical Architecture

### **Core Components**

#### **1. LLVM Visitor (`framec/src/frame_c/visitors/llvm_visitor.rs`)**
Main transpilation engine following existing visitor pattern:
- Frame AST → LLVM IR translation
- State machine optimization
- Platform-specific code generation
- Debug information integration

> **Current layout (2025-10-28)**: the visitor entry point now delegates to internal modules in `framec/src/frame_c/visitors/llvm/mod.rs` — `utils` (shared helpers), `value` (Frame→LLVM typing), `context` (system summaries), `builder` (IR emitter), and `visitor` (orchestration). This mirrors the long-term plan for a dedicated `framec/src/frame_c/llvm/` tree while keeping the code generation pipeline ready for multi-target growth.

#### **2. LLVM Backend Infrastructure (`framec/src/frame_c/llvm/`)**
```
framec/src/frame_c/llvm/
├── mod.rs                    # Module declarations
├── context.rs                # LLVM context management
├── codegen.rs                # Core code generation
├── runtime.rs                # Frame runtime generation
├── debug_info.rs             # DWARF debug info generation
├── types.rs                  # Frame-to-LLVM type mappings
├── optimization.rs           # LLVM optimization passes
├── platform.rs               # Platform-specific code
└── backend.rs                # Compilation pipeline
```

#### **3. Frame Runtime Library (`runtime/llvm/`)**
Native Frame runtime providing:
- Event dispatching kernel
- Memory management (reference counting)
- State machine utilities
- Platform-specific API implementations
- Cross-platform abstractions

#### **4. Testing Infrastructure (`framec_tests/llvm/`)**
Comprehensive test suite including:
- Positive tests (working Frame systems)
- Negative tests (error conditions)
- Performance benchmarks
- Cross-platform validation
- Integration with existing test framework

## Implementation Phases

### **Phase 1: Foundation & Minimal Viable Compiler (Weeks 1-4)**
**Goal**: Generate basic LLVM IR for simple Frame systems

#### **Week 1: Infrastructure Setup**
**Target Test**: `test_simple_system.frm`

**Deliverables:**
- Basic LLVM context management
- Skeleton LLVM visitor
- CLI integration for `-l llvm`
- Project structure setup

**Files Created:**
```
framec/src/frame_c/llvm/mod.rs
framec/src/frame_c/llvm/context.rs
framec/src/frame_c/llvm/types.rs
framec/src/frame_c/visitors/llvm_visitor.rs
```

**Technical Tasks:**
1. Add LLVM dependencies to `Cargo.toml`
2. Extend `TargetLanguage` enum with LLVM options
3. Create basic LLVM context wrapper
4. Implement skeleton visitor structure
5. Set up basic CLI integration

**Success Criteria:**
- `framec -l llvm simple.frm` executes without crashing
- Generates valid (minimal) LLVM IR output
- Integration tests pass with new CLI option

#### **Week 2: Basic System Structure**
**Target Test**: `test_simple_system.frm`

```frame
system Simple {
    machine:
        $Start {}
}
```

**Technical Tasks:**
1. Generate LLVM struct for Frame systems
2. Implement system constructor/destructor
3. Create basic event dispatch function
4. Handle system lifecycle management

**Generated LLVM IR Example:**
```llvm
%Simple = type { %FrameSystem }
%FrameSystem = type { i32 }  ; state

define %Simple* @simple_new() {
  ; System allocation and initialization
}

define void @simple_dispatch(%Simple* %self, i32 %event) {
  ; Basic dispatch framework
}
```

**Success Criteria:**
- Generates complete LLVM IR structure for simple systems
- IR compiles successfully with LLVM tools
- System structure matches Frame semantics

#### **Week 3: Basic Event Handling**
**Target Test**: `test_simple_event.frm`

```frame
system EventTest {
    interface:
        doSomething()
        
    machine:
        $Start {
            doSomething() {
                print("Event handled")
            }
        }
}
```

**Technical Tasks:**
1. Event enumeration generation
2. Event-to-handler dispatch logic
3. Interface method generation
4. Basic print statement support

**Success Criteria:**
- Interface methods generate proper LLVM functions
- Event dispatch works correctly
- Basic I/O operations function

#### **Week 4: State Transitions**
**Target Test**: `test_simple_transition.frm`

```frame
system TransitionTest {
    interface:
        start()
        
    machine:
        $Start {
            start() { -> $Running }
        }
        $Running {}
}
```

**Technical Tasks:**
1. State enumeration and management
2. Transition instruction generation
3. State-based event dispatching
4. State validation and error handling

**Success Criteria:**
- State transitions work correctly
- State-based event handling functions
- Invalid state transitions are handled properly

**Phase 1 Milestone**: Basic Frame state machines compile to working native code

### **Phase 2: Core Frame Semantics (Weeks 5-8)**
**Goal**: Support domain variables, actions, and complex event handling

#### **Week 5: Domain Variables** *(Status: Completed v0.86.22)*
**Target Tests**: `test_domain_variables.frm`, `test_variable_access.frm`

**Technical Focus:**
- Domain variable struct generation
- Variable access via GEP instructions
- Type system integration
- Memory layout optimization

**Key Features:**
```frame
domain:
    var count: int = 0
    var name: string = "default"
    var active: bool = true
```

**Generated LLVM:**
```llvm
%Counter = type { %FrameSystem, i32, i8*, i1 }
; base system + count + name + active
```

#### **Week 6: Actions Block** *(Status: Complete — return values shipped in v0.86.24)*

- ✅ Action functions now emit in LLVM (0.86.22) covering call statements and domain-variable mutations.
- ✅ Action-local variables lowered with type-aware allocas and assignments (0.86.22).
- ✅ Action parameters lowered with argument passing and type coercion (0.86.22).
- 🆕 Visitor refactor (v0.86.24) isolates action lowering inside the `builder` module for multi-target reuse.
- ✅ Explicit `return` values lowered for all supported scalar/string kinds (0.86.24).
**Target Tests**: `test_actions.frm`, `test_action_calls.frm`, `test_action_returns.frm`

**Technical Focus:**
- Action function generation
- Return value handling
- Local variable management
- Action-to-action calls

**Key Features:**
```frame
actions:
    calculate(x: int, y: int): int {
        var result = x + y
        return result
    }
```

#### **Week 7: Multiple States & Complex Dispatch** *(Status: Basic dispatcher & transitions working in v0.86.22; queue integration ongoing)*
**Target Tests**: `test_multi_state.frm`, `test_state_specific_handlers.frm`, `test_action_locals.frm`

**Technical Focus:**
- Multi-state dispatch optimization
- State-specific event handling
- Unhandled event management
- Performance optimization

#### **Week 8: Basic Runtime System**
**Target Tests**: Runtime execution tests (`basic/test_kernel_interop.frm`, `basic/test_parent_forward.frm`, `basic/test_parent_forward_queue.frm`)

**Deliverables:**
- Working Frame runtime library
- Memory management system
- Event system implementation
- Basic linking pipeline
- ✅ `frame_test_runner.py` invokes the LLVM smoke suite (`language_specific_llvm`) and links against `frame_runtime_llvm` (v0.86.22)
- ✅ LLVM smoke category wired into CI (GitHub Actions runs `python3 framec_tests/runner/frame_test_runner.py --languages llvm --categories language_specific_llvm --framec ./target/release/framec --timeout 30`)
- 🛠️ Queue scaffolding landed (v0.86.22) and now enqueues parent forwards via `frame_runtime_compartment_set_forward_event`; enter/exit forwarding and kernel pop helpers remain TODO.
- ✅ Generated LLVM code iterates queued events via `frame_runtime_event_is_message`; `test_parent_forward_queue.frm` validates single-pass parent forwarding while queued enter/exit handlers remain pending.

**Runtime Structure:**
```
runtime/llvm/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Runtime entry point
│   ├── memory.rs           # Reference counting
│   ├── event_system.rs     # Event management
│   └── frame_kernel.rs     # Core dispatch
├── include/
│   └── frame_runtime.h     # C headers
└── examples/
    └── counter.frm         # Example system
```

**Phase 2 Milestone**: Full Frame language core features work in native code

### **Phase 3: Advanced Features (Weeks 9-12)**
**Goal**: Support all Frame language features for production use

#### **Week 9: Enter/Exit Handlers**
**Target Tests**: `test_enter_exit.frm`, `test_hierarchical_states.frm`, *(planned)* `basic/test_enter_queue.frm`, `basic/test_exit_queue.frm`

**Technical Focus:**
- Enter/exit handler generation
- Handler sequencing with transitions
- Hierarchical state support
- State inheritance patterns

#### **Week 10: Exception Handling**
**Target Tests**: `test_exceptions.frm`, `test_try_catch.frm`, *(planned)* `advanced/test_exception_flow.frm`

**Technical Focus:**
- LLVM exception handling integration
- Try-catch block generation
- Exception propagation
- Error recovery mechanisms

#### **Week 11: External API Integration**
**Target Tests**: `test_file_operations.frm`, `test_process_spawning.frm`

**Technical Focus:**
- Platform-specific API implementations
- Cross-platform abstraction layer
- System call integration
- Foreign function interface

**Key APIs:**
- File I/O operations
- Process spawning and management
- Network communication
- Threading and concurrency

#### **Week 12: Optimization & Performance**
**Target Tests**: Performance benchmark suite

**Technical Focus:**
- LLVM optimization pass integration
- Frame-specific optimizations
- Performance profiling and tuning
- Benchmark validation

**Expected Performance:**
- **10-100x faster** than Python Frame
- **2-5x faster** than TypeScript Frame
- **Comparable** to hand-written C/C++

**Phase 3 Milestone**: Production-ready Frame LLVM backend with all language features

### **Phase 4: Production Features (Weeks 13-16)**
**Goal**: Debug support, cross-platform support, and production polish

#### **Week 13: Debug Information**
**Target Tests**: Debug information validation

**Technical Focus:**
- DWARF debug info generation
- Source location mapping
- Variable debug information
- Debugger integration testing

**Debug Features:**
- Frame source line debugging (not LLVM IR)
- Frame variable inspection
- Frame call stack display
- Integration with GDB/LLDB

#### **Week 14: Cross-Platform Support**
**Target Tests**: Cross-compilation validation

**Technical Focus:**
- Target triple support
- Platform-specific runtime builds
- Cross-compilation testing
- Platform API abstraction
- macOS first: validate clang/LLD pipeline on both Apple Silicon and Intel, then fan out to Linux/Windows

**Supported Platforms:**
- **x86_64**: Linux, macOS, Windows
- **ARM64**: Linux, macOS, iOS, Android
- **WebAssembly**: Browser and WASI targets
- **Embedded**: ARM Cortex-M, RISC-V

#### **Week 15: Build System Integration**
**Target Tests**: CLI and integration tests

**Technical Focus:**
- Complete CLI option implementation
- Linking pipeline automation
- Error message improvements
- Documentation integration

**CLI Features:**
```bash
framec -l llvm system.frm                    # Generate LLVM IR
framec -l llvm --emit-obj system.frm         # Generate object file
framec -l llvm --emit-exe system.frm         # Generate executable
framec -l llvm --target x86_64-linux system.frm # Cross-compile
framec -l llvm --optimize system.frm         # Enable optimizations
framec -l llvm --debug system.frm            # Include debug info
```

#### **Week 16: Testing & Documentation**
**Target Tests**: Complete test suite validation

**Technical Focus:**
- Full test suite pass rate validation
- Performance benchmarking and documentation
- User documentation completion
- Integration guide creation

**Documentation Deliverables:**
- LLVM backend user guide
- Performance tuning guide
- Cross-compilation guide
- Debugging integration guide

**Phase 4 Milestone**: Production-ready LLVM backend with complete toolchain integration

## Test-Driven Development Strategy

### **Test Categories**

#### **Core Language Tests**
```
framec_tests/llvm/src/positive_tests/
├── basic/
│   ├── test_simple_system.frm
│   ├── test_simple_event.frm
│   ├── test_simple_transition.frm
│   └── test_basic_interface.frm
├── core_features/
│   ├── test_domain_variables.frm
│   ├── test_actions.frm
│   ├── test_multi_state.frm
│   └── test_variable_access.frm
├── advanced/
│   ├── test_enter_exit.frm
│   ├── test_exceptions.frm
│   ├── test_hierarchical_states.frm
│   └── test_external_apis.frm
└── production/
    ├── test_debug_info.frm
    ├── test_cross_platform.frm
    └── test_optimization.frm
```

#### **Performance Benchmarks**
```
framec_tests/llvm/src/performance_tests/
├── event_dispatch_benchmark.frm      # Event handling speed
├── state_transition_benchmark.frm    # Transition performance
├── memory_allocation_benchmark.frm   # Memory management speed
├── external_api_benchmark.frm        # System call overhead
└── complex_system_benchmark.frm      # Real-world system performance
```

#### **Integration Tests**
```
framec_tests/llvm/src/integration_tests/
├── vs_python_compatibility.frm       # Semantic equivalence
├── vs_typescript_compatibility.frm   # Cross-language consistency
├── debugger_integration.frm          # GDB/LLDB integration
└── toolchain_integration.frm         # Build system integration
```

### **Testing Framework Integration**

#### **Test Runner Extensions**
```python
# framec_tests/runner/frame_test_runner.py
class LLVMTestRunner:
    def run_llvm_test(self, test_file):
        # 1. Compile Frame to LLVM IR
        ir_result = self.compile_to_llvm_ir(test_file)
        
        # 2. Compile LLVM IR to object file
        obj_result = self.compile_ir_to_object(ir_result.ir_file)
        
        # 3. Link with Frame runtime
        exe_result = self.link_with_runtime(obj_result.obj_file)
        
        # 4. Execute and capture output
        run_result = self.execute_native_binary(exe_result.exe_file)
        
        # 5. Compare with expected output
        return self.validate_output(run_result, test_file.expected)
```

#### **Performance Validation**
```python
class PerformanceTester:
    def benchmark_llvm_vs_python(self, test_file):
        # Run same Frame system in both backends
        python_time = self.time_python_execution(test_file)
        llvm_time = self.time_llvm_execution(test_file)
        
        speedup = python_time / llvm_time
        assert speedup >= 10.0, f"Expected 10x speedup, got {speedup}x"
        
        return speedup
```

### **Continuous Integration Strategy**

#### **Weekly Success Criteria**
Each week has quantifiable pass/fail criteria:

**Week 1**: Infrastructure
- ✅ `framec -l llvm` executes without crashes
- ✅ Generates valid LLVM IR (even if minimal)
- ✅ CI pipeline includes LLVM tests

**Week 2**: Basic Systems  
- ✅ `test_simple_system.frm` passes
- ✅ Generated IR compiles with `llc`
- ✅ System structure matches specification

**Week 3**: Event Handling
- ✅ `test_simple_event.frm` passes
- ✅ Interface methods work correctly
- ✅ Event dispatch functions properly

**Week 4**: State Transitions
- ✅ `test_simple_transition.frm` passes
- ✅ State changes work correctly
- ✅ Invalid transitions handled properly

**Weeks 5-16**: Similar criteria for each phase

#### **Regression Prevention**
- **All existing Python/TypeScript tests** continue to pass
- **Performance baselines** established and maintained
- **Cross-platform validation** on every commit
- **Memory safety verification** with sanitizers

## Risk Mitigation

### **Technical Risks**

#### **Risk 1: LLVM Complexity**
**Mitigation:**
- Start with minimal LLVM IR generation
- Use `inkwell` safe wrapper instead of raw LLVM-C API
- Incremental complexity increase
- Extensive testing at each step
- The new modular builder keeps string-based emission contained, making a future inkwell migration a drop-in replacement for the `builder` module.

#### **Risk 2: Performance Expectations**
**Mitigation:**
- Establish realistic performance baselines early
- Continuous benchmarking throughout development
- Focus on correctness before optimization
- Performance validation against native C equivalents

#### **Risk 3: Cross-Platform Compatibility**
**Mitigation:**
- Target single platform (Linux x86_64) initially
- Platform abstraction layer design from start
- CI testing on all target platforms
- Platform-specific test suites

#### **Risk 4: Debug Information Complexity**
**Mitigation:**
- Basic DWARF generation first
- Incremental debug feature addition
- Integration testing with GDB/LLDB
- Fallback to basic debugging if needed

### **Project Risks**

#### **Risk 1: Timeline Overrun**
**Mitigation:**
- Conservative estimates with buffer time
- Weekly milestone validation
- Scope reduction options identified
- Incremental delivery approach

#### **Risk 2: Integration Difficulties**
**Mitigation:**
- Regular integration with main branch
- Existing visitor pattern reduces integration risk
- Extensive testing of integration points
- Rollback procedures defined

#### **Risk 3: Resource Requirements**
**Mitigation:**
- Optional feature flag for LLVM support
- Minimal changes to existing codebase
- Documentation for team onboarding
- Expert consultation available

## Success Metrics

### **Functional Metrics**
- **Test Pass Rate**: 100% of Frame language tests pass in LLVM backend
- **Feature Completeness**: All Frame language features supported
- **Platform Coverage**: Native compilation for 5+ target platforms
- **Debug Integration**: Full GDB/LLDB debugging support

### **Performance Metrics**
- **Python Speedup**: 10-100x faster than Python Frame implementations
- **TypeScript Speedup**: 2-5x faster than TypeScript Frame implementations
- **Memory Efficiency**: 90% reduction in memory usage vs Python
- **Startup Time**: Sub-millisecond Frame system initialization

### **Quality Metrics**
- **Memory Safety**: Zero memory leaks in reference implementation
- **Cross-Platform Consistency**: Identical behavior across all platforms
- **Backward Compatibility**: 100% semantic compatibility with existing Frame
- **Documentation Coverage**: Complete user and developer documentation

### **Adoption Metrics**
- **Performance Benchmarks**: Public benchmarks vs popular alternatives
- **Use Case Validation**: Real-world systems deployed successfully
- **Developer Experience**: Positive feedback on ease of use
- **Ecosystem Integration**: Works with existing Frame toolchain

## Future Roadmap

### **Post-Implementation Enhancements**

#### **Advanced Optimization (Future Release)**
- **Profile-Guided Optimization**: Runtime profiling for hot path optimization
- **Link-Time Optimization**: Whole-program optimization
- **Custom Optimization Passes**: Frame-specific LLVM passes
- **SIMD Integration**: Vectorized operations for parallel state machines

#### **Specialized Backends (Future Releases)**
- **GPU Compilation**: CUDA/OpenCL generation for parallel Frame systems
- **Embedded Optimization**: Size-optimized code for microcontrollers
- **Real-Time Variants**: Hard real-time Frame systems
- **JIT Compilation**: Runtime Frame system compilation

#### **Advanced Debugging (Future Release)**
- **Time-Travel Debugging**: Record/replay Frame system execution
- **Visual State Machine Debugging**: Graphical state machine visualization
- **Performance Profiling**: Integrated performance analysis tools
- **Multi-Language Debugging**: Seamless Frame ↔ C/C++ debugging

#### **Ecosystem Integration (Future Releases)**
- **Package Management**: Native Frame library system
- **IDE Integration**: Enhanced VS Code/CLion support
- **Static Analysis**: Frame-specific static analysis tools
- **Formal Verification**: Mathematical verification of Frame systems

## Conclusion

The Frame LLVM backend represents a significant enhancement to the Frame ecosystem, enabling native performance while preserving Frame's unique state machine semantics. The implementation plan provides a structured, test-driven approach that minimizes risk while delivering incremental value.

**Key Benefits:**
- **Performance**: Native execution speed competitive with C/C++
- **Debugging**: Full native debugging support with Frame semantics
- **Deployment**: Self-contained native binaries with no runtime dependencies
- **Platform Support**: Cross-platform native compilation
- **Integration**: Seamless integration with existing Frame toolchain

**Strategic Impact:**
- Positions Frame as a systems programming language
- Enables Frame adoption in performance-critical applications
- Provides research platform for state machine optimization
- Establishes Frame as unique in combining modeling clarity with native performance

This implementation plan provides the roadmap for making Frame a competitive choice for high-performance systems development while maintaining the clarity and ease of use that makes Frame distinctive.

---

*Implementation plan for Frame LLVM backend*  
*Target: Native code generation with full Frame semantics*  
*Timeline: 16 weeks for complete implementation*  
*Approach: Test-driven development with incremental delivery*

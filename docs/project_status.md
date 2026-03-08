# Frame Transpiler Project Status

## Current State

**Branch**: `going_native`  
**Version**: `v0.86.50`  
**Status**: ✅ Python 462/462 · ✅ TypeScript 433/433 · ✅ LLVM smoke 18/18  
**Achievement**: LLVM backend now propagates typed interface arguments through queued dispatch and parent forwards; stack multi-pop semantics validated end-to-end.  
**Latest Snapshot (2025-10-28)**: 913 specs passing (common + language-specific + LLVM smoke) with the LLVM suite covering enter/exit, parent forwarding, event parameters, and multi-pop pops.  
**Rust Version**: 1.89.0 (Latest Stable)

## Latest Updates (November 18, 2025)

### 🛠️ Frame v0.86.49 - TypeScript Runtime Types and CLI Bug Fixes
- **Runtime Type Alignment (TypeScript)**: Added `frame_runtime_ts/index.d.ts` so that the published TypeScript runtime types (`FrameEvent`, `FrameCompartment`) match the actual runtime implementation and V3 generator usage (multi-argument `FrameCompartment` constructor, `enterArgs`, `stateArgs`, and `message` fields).
- **Bug #078 Fixed (TS runtime d.ts mismatch)**: Generated TypeScript now compiles cleanly against the bundled runtime types; constructor arity and property accesses are aligned with the new `.d.ts`.
- **Bug #073 Fixed (duplicate methods per state, reopen)**: Confirmed the V3 TS CLI emitter still generates a single public method per interface with state-based routing; with the new runtime types, the minimal module validator now passes without TypeScript errors.
- **Bug #074 Fixed (actions/domain emits, reopen)**: Verified that actions and domain fields are emitted correctly in CLI modules and that TypeScript compilation succeeds against the updated runtime types.
- **V3 CLI Suites Green (TS)**: `language_specific/typescript/v3_cli` (including `multi_state_interface_router` and `actions_and_domain_emit_issues`) passes fully under `@tsc-compile` validation.

### 🛠️ Frame v0.86.50 - V3 TypeScript Interface Wrappers and Router Parity
- **Public Interface Wrappers (TS)**: V3 TypeScript systems now expose consumer-friendly public methods for interface events (e.g., `start()`, `runtimeMessage(payload)`) that construct `FrameEvent` instances and call `_frame_router` without requiring callers to pass `FrameEvent`/`FrameCompartment`.
- **Functional Router (TS)**: `_frame_router` now dispatches on `__e.message` and delegates to private `_event_*` handlers that switch on `c.state`, restoring parity with the V3 design documented in `architecture_v3/codegen.md`.
- **Bug #079 Fixed**: Generated TS modules no longer expose internal runtime parameters on public methods, and CLI fixtures validate that wrapper signatures and router dispatch match expectations (`v3_cli` remains 100% green).

### 🛠️ Frame v0.86.47 - V3 Outer Pipeline (PT) Aligned with AST
- **AST-Backed Outer Pipeline (PT)**: Systems, block ordering, machine/state headers, handler placement, system parameters, and domain semantics are now driven by `SystemParserV3`, `MachineParserV3`, `DomainBlockScannerV3`, `ModuleAst`, and `Arcanum` instead of ad-hoc byte scans.
- **PT Suites 100% Green**: `all_v3` categories for Python and TypeScript remain fully passing after the refactor, confirming that V3 semantics and tests are aligned.
- **Docs Updated**: V3 grammar, language guide, runtime, and capability matrix now describe the AST/scanner architecture as the authoritative model for PT; previous v0.86.x entries below remain as historical context.

### 🛠️ Frame v0.86.25 - LLVM Queue Semantics & Typed Events
- **Typed Event Payloads**: `FrameEvent` now stores `StateValue`s with FFI push/get helpers so interface parameters travel through queued dispatch, parent forwarding, and re-entrancy without loss.
- **Builder Updates**: Interface dispatch signatures include parameter metadata; queued handlers pull arguments back via runtime getters, and parent forwards preserve payloads before re-enqueuing.
- **Main Function Support**: LLVM backend evaluates interface arguments in `main`, including domain reads and literal expressions, matching interface arity/type expectations.
- **Expanded Smoke Suite**: Added `test_event_parameters.frm` and `test_state_stack_multi_pop.frm` alongside earlier fixtures; LLVM category now passes 18/18 tests covering state params, enter args, parent forwards, and stack pops.
- **Docs & Planning**: README, HOW_TO, and planning docs updated with the new test counts and backend capabilities.

### ✅ Previous Updates (October 19, 2025)

#### Frame v0.85.6 - Critical TypeScript Bug Fixes Complete
- **Bug #52 RESOLVED**: Interface method calls now generate `this.method()` correctly in TypeScript
- **Bug #53 RESOLVED**: Exception variables now generate `e` not `this.e` in TypeScript  
- **Regression Tests**: Added targeted fixtures for both bug classes
- **Result**: TypeScript generation unblocked VS Code extension development; execution rate subsequently tracked separately (see current snapshot)

### ✅ Toolchain & Dependency Updates (January 27, 2025)
- **Rust**: Updated from 1.83.0 → 1.89.0 (latest stable)
- **Edition**: Upgraded from Rust 2018 → 2021
- **All Dependencies**: Updated to latest versions
- **Build Status**: Zero warnings, zero deprecations
- **Test Suite**: 100% passing (341/341 tests)

### ✅ Frame v0.56 Python Enhancement Features
- **Walrus Operator (:=)**: Assignment expressions for inline variable creation
- **Numeric Literal Underscores**: `1_000_000`, `0xFF_FF` for readability
- **Complex Numbers**: `3+4j`, `2.5j` imaginary number support
- **Type Aliases**: Python 3.12+ style `type MyType = int`
- **Scientific Notation**: `1.23e10`, `6.022e23` exponential notation

### ✅ Frame v0.55 State Parameters & Type Annotations
- **State Parameters**: States can receive and store parameters during transitions
- **Type Annotations**: Full support for parameter and return type hints
- **Property Decorators**: `@property` for computed properties in classes
- **100% Test Success**: All 339 tests passing (milestone achievement)

### ✅ Frame v0.54 Collection Operations
- **Star Expressions**: Unpacking with `*` in assignments and calls
- **Collection Constructors**: `list()`, `dict()`, `set()`, `tuple()`
- **Tuple Unpacking**: Multiple assignment support

### ✅ Frame v0.53 Multiple Assignment
- **Multiple Variable Declaration**: `var x, y, z = 1, 2, 3`
- **Tuple Unpacking**: `var (a, b) = get_pair()`
- **Chained Assignment**: `x = y = z = 0`

## Complete Feature Set (v0.30 - v0.56)

### Core Language Features
- **State Machines**: Hierarchical state machines with enter/exit handlers
- **Multi-Entity Support**: Multiple functions and systems per module
- **Module System**: Named modules with qualified access
- **Classes**: Object-oriented programming with methods and variables
- **Async/Await**: Full async function and event handler support
- **Pattern Matching**: Complete match-case support with guards
- **Generators**: Regular and async generators with yield

### Python Operator Alignment (100% Complete)
- **All Logical Operators**: `and`, `or`, `not`
- **All Bitwise Operators**: `&`, `|`, `~`, `^`, `<<`, `>>`
- **All Compound Assignments**: `+=`, `-=`, `*=`, `/=`, `%=`, `**=`, etc.
- **Identity/Membership**: `is`, `is not`, `in`, `not in`
- **Matrix Multiplication**: `@` and `@=` operators
- **Floor Division**: `//` and `//=`
- **Exponentiation**: `**` with right associativity

### Python String Support
- **F-strings**: Formatted string literals with expressions
- **Raw strings**: No escape sequence processing
- **Byte strings**: Binary data representation
- **Triple-quoted**: Multi-line strings
- **Percent formatting**: Classic Python string formatting

### Advanced Features
- **List/Dict/Set Comprehensions**: Full Python-style comprehensions
- **Slicing**: Complete Python-style slicing support
- **With Statements**: Context manager support
- **Try-Except**: Exception handling with finally
- **Assert Statements**: Runtime assertions
- **Del Statement**: Explicit deletion
- **Global Keyword**: Explicit global access
- **Enums**: Custom values, string enums, iteration
- **Import Statements**: Native Python imports

### Native Python Operations
- **Type Conversions**: `str()`, `int()`, `float()`, `bool()`
- **List Operations**: All Python list methods work natively
- **String Operations**: All Python string methods work natively
- **Dict Operations**: All Python dict methods work natively
- **No Import Required**: Python built-in functions work directly in Frame

## Test Infrastructure

### Test Runner
- **Location**: `framec_tests/runner/frame_test_runner.py`
- **Coverage**: Python systems (462), TypeScript systems (433), LLVM smoke fixtures (18), Negative suite (14)
- **Command**: `python3 framec_tests/runner/frame_test_runner.py --languages python typescript llvm --framec ./target/release/framec`
- **Success Rate**: 100%

### Reports
- **Test Log**: `framec_tests/reports/test_log.md`
- **Test Matrix**: `framec_tests/reports/test_matrix_v0.31.md`
- **JSON Results**: `framec_tests/reports/test_results_v0.31.json`

## Documentation Structure

### Frame Language Design
- **Location**: `docs/framelang_design/`
- **Grammar**: `grammar.md` - Complete BNF specification (v0.56)
- **Dev Notes**: `dev_notes.md` - Version history and features

### Going Native Roadmap
- **Roadmap**: `docs/framepiler_design/going_native/roadmap.md` — phased plan for native targets and shared debug infrastructure

### Achievement Documents
- **Location**: `docs/`
- **Files**: `v0.30_achievements.md` through `v0.56_achievements.md`
- **Coverage**: Complete documentation of all features per version

## Build Instructions

```bash
# Build transpiler
cargo build --release

# Run tests
cd framec_tests
python3 runner/frame_test_runner.py --all --framec ../target/release/framec

# Transpile example
./target/release/framec -l python_3 file.frm
```

## Language Support

### 1st Class (Full Implementation)
- **Python 3**: Complete visitor with all features (341/341 tests passing)
- **TypeScript**: Complete visitor with all features (423/423 tests passing)

### 2nd Class (Design Guides / Archived)
- C/C++, JavaScript, C#, Java, Go (reference-only)
- Rust (legacy visitor removed; retained for historical planning)

### Target Options
- `python_3`: Python code generation
- `typescript`: TypeScript code generation
- `graphviz`: DOT-based state diagram generation

## Project Metrics

- **Versions Released**: 55 (v0.30 through v0.84.0)
- **Test Files**: 764 (341 Python + 423 TypeScript)
- **Success Rate**: 100% (both languages)
- **Language Features**: Complete coverage of Python 3.8+ and TypeScript ES2020+ features
- **Code Quality**: Zero warnings, zero deprecations
- **Rust Edition**: 2021
- **Min Rust Version**: 1.89.0

## Next Steps

1. **LLVM Enhancements**: Native codegen improvements via LLVM backend
2. **Cross-Language Architecture**: Expand `@target` diagnostics and native block support
3. **Build System**: Package management and dependency resolution
4. **Performance**: Optimization of transpiler performance
5. **IDE Support**: Enhanced VSCode extension features
6. **C Backend (Phase 1)**: Implement minimal C emitter with runtime FFI and smoke-suite parity — see `docs/framepiler_design/going_native/c_backend_plan.md` and `docs/framelang_design/target_language_specifications/c/c_body_grammar.md`
7. **C++ Backend (Phase 1)**: Minimal C++ emitter using `extern "C"` runtime FFI — see `docs/framepiler_design/going_native/cpp_backend_plan.md` and `docs/framelang_design/target_language_specifications/cpp/cpp_body_grammar.md`
8. **Java Backend (Phase 1)**: Java emitter + JNI shim to native runtime — see `docs/framepiler_design/going_native/java_backend_plan.md` and `docs/framelang_design/target_language_specifications/java/java_body_grammar.md`
9. **Rust Backend (Phase 1)**: Rust emitter with safe wrappers over the native runtime — see `docs/framepiler_design/going_native/rust_backend_plan.md` and `docs/framelang_design/target_language_specifications/rust/rust_body_grammar.md`

## Contact

- **Creator**: Mark Truluck
- **Discord**: [The Art of the State](https://discord.com/invite/CfbU4QCbSD)
- **Documentation**: [Read the Docs](https://docs.frame-lang.org)
- **Playground**: [Frame Playground](https://playground.frame-lang.org)
- **Issues**: bugs@frame-lang.org

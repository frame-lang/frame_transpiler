# Cross-Language Support Implementation Plan

**Document Version**: 1.0  
**Date**: 2025-10-30  
**Status**: Implementation Plan  
**Priority**: High  
**Related Issues**: Bug #055 - TypeScript async runtime lacks socket helpers

## Executive Summary
``
This plan implements target-specific syntax support in Frame using `@target` declarations, enabling native language constructs while preserving Frame's universal state machine patterns. The approach solves Bug #055 immediately and provides a scalable architecture for future cross-language challenges. The immediate focus is on Python and TypeScript runtime parity; LLVM visitor integration resumes once those milestones land.

**Key Outcome**: Frame evolves from "universal syntax" to "universal state machine patterns with target-specific implementation."

## 🎯 Project Goals

### Primary Objectives
1. **Solve Bug #055**: Enable TypeScript async socket operations without runtime helpers
2. **Eliminate N×M maintenance**: Remove visitor updates for every runtime feature
3. **Enable native performance**: Allow target-optimized implementations
4. **Preserve Frame semantics**: Maintain universal state machine patterns
5. **Establish toolchain clarity**: Define which tools own compilation artifacts for each target
6. **Implement first-class diagnostics**: Dual Frame/target line number reporting from day one
7. **Ensure robust boundary detection**: Handle nested constructs reliably in scanner transitions
8. **Create governance framework**: Prevent excessive target fragmentation through bounded usage

### Success Metrics
- [ ] Bug #055 TypeScript async socket operations compile and execute
- [ ] Reduced visitor complexity (measured by lines of runtime helper code)
- [ ] Performance parity with hand-written target code
- [ ] 100% Frame test suite compatibility

## 📋 Implementation Phases

### **Phase 1: Foundation (Week 1-2)**
*Establish target declaration parsing and basic infrastructure*

#### Week 1: Scanner Extensions & Toolchain Strategy
**Goal**: Add `@target` syntax support and define toolchain ownership

**Tasks**:
- [x] **LLVM Toolchain Decision**: Prototype both runtime FFI shims and raw LLVM IR approaches *(resolved: ship FFI shim first, then migrate to pure IR after feature parity)*
- [x] **Document toolchain ownership**: Define which tools compile artifacts for each target *(FFI runtime owned by Rust crate; LLVM codegen emits shim calls until IR replacement phase)*
- [x] Add dedicated `TargetAnnotation` token and keyword handling to `TokenType`
- [x] Implement `scan_target_declaration()` method in scanner *(handled inline during header sweep)*
- [x] Add `ScanningMode` enum with `TargetDiscovery`, `FrameCommon`, `TargetSpecific` variants
- [x] Extend `Scanner` struct with target-aware fields (target language, mode scaffolding)
- [x] Add `switch_scanning_mode()` method with robust boundary detection

*Status Update (2025-10-30)*:
- Scanner now transitions between discovery/common/target modes and records raw target blocks as `TargetRegion` entries, giving us per-language slices for follow-on parsing.
- Single-file, CLI validation, and multi-file compilers share the captured regions via `Arc`, allowing both passes to access the same metadata.
- ✅ All Week 1 tasks completed: toolchain decision documented (FFI-first), ownership noted, boundary detection validated.

**Deliverables**:
- **Toolchain strategy document** defining compilation artifacts ownership (FFI-first; migrate to pure IR after feature parity)
- **LLVM approach prototype** (FFI shim vs raw IR) with recommendation – **resolved: ship FFI shim first, plan staged IR migration**
- Modified `scanner.rs` with target declaration support
- CLI / build tooling source `@target` declarations and reject conflicting overrides
- **Boundary detection test suite** covering nested braces, strings, comments
- Unit tests for `@target typescript` parsing
- Integration tests with existing Frame scanner

**Validation Criteria**:
- LLVM toolchain strategy decided and documented (FFI-first path recorded)
- Scanner recognizes `@target typescript` at file start
- Boundary detection handles complex nested constructs correctly
- Backward compatibility: files without `@target` work unchanged
- Error handling for invalid target names

#### Week 2: Parser Infrastructure & Diagnostics Integration
**Goal**: Extend parser to handle target-specific syntax regions with first-class diagnostics

**Tasks**:
- [x] **Diagnostics integration**: Wire dual line number reporting through existing diagnostic pipeline
- [x] Add `TargetDiscoveryPass` struct and implementation
- [x] Extend `ActionBody` enum with `TargetSpecific` variant
- [x] Implement `TargetRegion` and `TargetSourceMap` for diagnostics
- [x] Add boundary detection logic (`detect_frame_boundary()`)
- [x] Create `UnrecognizedStatement` AST node type
- [x] **Enhanced error reporting**: Implement Frame + target line number display

*Status Update (2025-10-30)*:
- `TargetRegion` snapshots and source-map scaffolding land in the AST; parser now preserves them for diagnostics work in Week 2.
- Dedicated `TargetDiscoveryPass` maps Frame vs native spans and feeds both compilation passes; diagnostic wiring + native AST integration remain.
- Event handlers retain target block metadata (`target_specific_regions`) so future native parsers/codegen can recover raw source slices without inflating the existing statement pipeline.
- Python visitor now consumes stored `target_specific_regions`, allowing native Python snippets to be emitted once scanner captures the regions.
- Body classification now flows through the AST (`ActionBody`), and nodes capture unsupported target regions as `UnrecognizedStatementNode`s for downstream diagnostics.
- Python visitor centralizes native emission through the new body metadata, producing deterministic ignore notes when other targets are present.
- Parse errors now surface both frame and target locations (with snippets) throughout CLI and module compiler flows, giving users consistent dual-line diagnostics.
- CLI and build tooling respect module-level `@target` / `#[target: ...]` directives, so inline declarations no longer trigger “No target language specified.”

**Deliverables**:
- **Integrated diagnostics system** with dual line number support
- Modified `parser.rs` with 3-pass architecture
- `TargetRegion` implementation with source mapping
- **Comprehensive error message examples** showing Frame + target locations
- Parser tests for target-specific action bodies
- Validation pass covering source-map emission + AST dumps to confirm diagnostics stay aligned after Week 4 visitor integration
- Post-visitor regression pass verifying CLI `--debug-output` source maps and AST dump tooling for native block scenarios

**Validation Criteria**:``
- **Diagnostics show both Frame and target line numbers** in error messages
- Parser correctly identifies Frame vs target-specific regions
- Raw target tokens stored for later processing
- Source line mapping works for error reporting
- Error messages follow established Frame diagnostic format

### **Phase 2: Target-Specific Processing (Week 3-4)**
*Implement native language syntax integration*

#### Week 3: Target Parsers
**Goal**: Add TypeScript and Python syntax parsing for target blocks

**Tasks**:
- [x] Create `TargetAst` trait and implementations
- [x] Implement `TypeScriptParser::parse_statement()` method
- [x] Implement `PythonParser::parse_statement()` method  
- [x] Add `resolve_target_statements()` parser method
- [x] Implement dual-language error reporting

**Python Target Workstream (active)**
- [x] Design Python target parser:
- [x] Convert Python target AST (`Suite`) into native statement nodes so visitors can emit structured code
- [x] Replace legacy Python-specific parsing paths with target parser outputs (actions/functions/handlers)
 review target-region plumbing and outline required AST/data structures
- [x] Implement Python parser module and `TargetAst` integration; update shared parser to invoke it for `#[target: python]` blocks
- [x] Update Python visitor/tests to consume the new native AST and document plan progress

**Design Outline (Python)**
- Introduce `framec/src/frame_c/target_parsers/` with a shared `TargetAst` trait (`target_language()`, `to_source()`, `diagnostics()`).
- Add `PythonTargetParser` that dedents region content, parses it with `rustpython_parser`, and returns a `PythonTargetAst` (captures `Suite`, normalized source text, and per-node offsets). **Status**: landed with basic suite parsing plus error propagation tests.
- Extend `EventHandlerNode` with `parsed_target_blocks: Vec<ParsedTargetBlock>` holding `(region_ref, Arc<dyn TargetAst>>)` so both raw-region references and parsed AST are available for diagnostics and generation. **Status**: parser now attaches typed blocks while preserving raw references for other targets.
- Extend `ActionNode` in the same fashion so actions honor target-specific code paths before falling back to Frame statements. **Status**: implemented; Python visitor consumes parsed blocks for actions and emits notes for skipped targets.
- Extend `FunctionNode` and `OperationNode` with the same metadata so helper functions and operations dispatch through native target blocks before Frame fallbacks. **Status**: parser + Python visitor updated; helper fixture now exercises inline Python in a global function.
- Wire `Parser::resolve_target_specific_blocks` to call the registry, translate `TargetParseError` into Frame `ParseError`, and attach diagnostics (Frame line + target line). **Status**: implemented for Python (unsupported targets skipped); errors now echo both the offending target line (with snippet) and the frame line.

- `target_parsers/python.rs` integrates `rustpython_parser` (with location support) + unit coverage for both happy/errant snippets, verifying target-line diagnostics.
- Event handlers/functions/actions/operations all carry `parsed_target_blocks`, and `python_visitor_v2` emits annotated comments (`[target … -> frame …]`) ahead of native blocks while noting ignored targets deterministically.
- Target-specific Python blocks are now segmented into statement/whitespace elements with preserved frame-line mapping, and the visitor emits each segment directly instead of falling back to raw dedented strings (validated by `cargo test -p framec target_parsers::python` and full `frame_test_runner` for Python).
- Parser now resolves Frame statements against captured target regions, filtering out overlapping statements before classification so mixed bodies don't duplicate native code; exercised via the same unit suite plus end-to-end Python runner.
- Added a minimal TypeScript target parser that dedents segments and exposes structured elements, updated the TypeScript visitor to emit native blocks with metadata, and introduced `test_target_native_block.frm` to cover the flow (current TypeScript suite: 436/437 passing; only `test_file_io` still fails due to the existing file-I/O runtime gap).
- Release build succeeds (`cargo build --release`), and `cargo test -p framec target_parsers::python python_visitor_v2::tests` plus `cargo test -p framec target_parsers::typescript` validate the parser + visitor pipeline.
- Shared runtimes are now aligned: Python targets import `frame_runtime_py`, TypeScript multifile generation imports the new `frame_runtime_ts` package, and CLI/build tooling emit both packages. Remaining work is isolated to `FrameSocketClient` async helpers and debugger harness coverage.

**Deliverables**:
- Target-specific parser modules
- AST nodes for native language constructs
- Error reporting with Frame + target line numbers

**Validation Criteria**:
- TypeScript action bodies parse correctly
- Python action bodies parse correctly
- Error messages show both Frame and target locations

#### Week 4: Visitor Integration & Runtime Alignment
**Goal**: Update visitors to handle target-specific AST nodes while ensuring runtimes/FSL continue to own Frame semantics

- [x] Modify TypeScript visitor to output target-specific blocks directly
- [x] Modify Python visitor to output target-specific blocks directly
- [x] Implement `TargetAst::to_code()` methods
- [x] Extract TypeScript runtime helpers into shared `frame_runtime_ts` module and switch visitor/linker to imports
- [x] Package `frame_runtime_ts` with CLI/multifile builds (mirroring `frame_runtime_py`)
- [x] Validate that generated code delegates kernel/state semantics to runtime/FSL helpers
- [x] Update visitor tests for target-specific syntax

*Status Update (2025-10-31)*:
- TypeScript visitor now routes action invocations through the public wrappers; native block fixture `framec_tests/language_specific/typescript/test_target_native_block.frm` exercises the path and the TypeScript suite reports 5/5 passes.
- Updated call-chain lowering strips the `_action_` prefix when present, keeping event-system bindings (which still target the private `_action_` symbols) intact.
- Confirmed `frame_test_runner.py --languages typescript --categories language_specific_typescript` succeeds post-change; LLVM visitor work resumes after Python/TypeScript async infrastructure lands.
- Target parsers expose `to_code()`; both Python and TypeScript visitors use it as the fallback path, and unit suites cover the new helper.
- TypeScript output now imports the shared `frame_runtime_ts` module (CLI + `frame_build` emit `frame_runtime_ts/index.ts`; single-file generation still inlines the runtime bundle for convenience).
  - Migration steps completed: `frame_runtime_ts/index.ts` added; visitor/linker import path switched to `./frame_runtime_ts`; build tooling drops the module next to generated artifacts.
  - Validation: rerun `frame_test_runner.py --languages python typescript --framec ./target/release/framec`, add smoke fixture `framec_tests/language_specific/typescript/runtime/test_runtime_import.frm`, and document packaging change (CLI + HOW_TO + roadmap). ✅ (`python3 framec_tests/runner/frame_test_runner.py --languages python typescript --framec ./target/release/framec`)
- Dual-language error reporting wired through `ParseError::to_display_string`; regression test validates frame + target line context on native parse failures.
- Frame runtime now includes a functional `FrameSocketClient` (connect/readLine/writeLine/close). Import statements are wired for multifile builds; next step is to hook the runtime protocol spec and add the Node echo-server harness.

**Deliverables**:
- Updated visitor implementations
- **LLVM visitor implementation** using chosen approach
- Native code generation from target-specific AST
- Confirmation that runtime/FSL helpers remain the authoritative implementation of Frame semantics
- Comprehensive visitor test coverage

**Validation Criteria**:
- Generated TypeScript compiles without errors
- Generated Python executes without errors
- Generated code continues to lean on runtime/FSL APIs for state management across targets

### **Phase 2.5: Native Import Surface & FID Pipeline (Week 5)**
*Shift from manual declarations to automatic discovery + generated `.fid` files*

**Goal**: Treat native import statements as first-class citizens, cache the source lines for diagnostics, and prepare the auto-generated inspection pipeline that will emit `.fid` metadata per target.

**Completed Work**:
- [x] Parser records native import statements as `ImportType::Native` (see `framec/src/frame_c/parser.rs`) and captures the original source text for visitors/diagnostics.
- [x] Compiler/CLI pass `Arc<Vec<String>>` source buffers into both parser passes so native snippets and error messages include target-line context.
- [x] Python + TypeScript visitors replay native imports directly (non-target visitors surface a comment explaining the skip).
- [x] TypeScript fixtures updated to rely on native imports instead of `#[target: typescript]` blocks; full transpile-only suite is green.
- [x] Documentation rewritten (`docs/framelang_design/native_imports_and_fid.md`) describing the native-import-first workflow and the follow-on `.fid` generation.

**Still To Do**:
- [ ] Wire the declaration generator (`framec decl`) to consume captured imports: invoke adapter tooling (TypeDoc / Python `inspect`) and emit `.fid` cache entries automatically.
- [ ] Load generated `.fid` metadata during compilation so specs receive symbol/type checking without hand-authored declarations.
- [ ] Produce diagnostics when an import resolves to a missing `.fid` entry (stale cache, missing runtime implementation, etc.).
- [ ] Document the `.fid` cache layout + lifecycle in HOW_TO and developer docs once the generator lands.

**Validation Criteria**:
- Frame CLI regenerates `.fid` files on demand and respects cache invalidation (timestamp/hash).
- Parser + visitors continue to succeed with multi-line / aliased native imports.
- Missing or incompatible runtime signatures produce actionable compiler errors tied to both Frame and target source lines.

### **Phase 2.6: Declaration Generator Tooling (Week 6)**
*Automate creation of native module contracts from existing language metadata*

**Goal**: Provide an opt-in tool that converts target-specific signature sources (e.g., `.d.ts`, Python stubs) into Frame `native module` declarations, so teams are not forced to hand-maintain the contracts.

#### Week 6a: Generator Scaffolding & CLI
- [ ] Add `framec decl import` subcommand (alias `framec declarations import`) with discoverable help text.
- [ ] Define generator config (`.frame_declgen.json`) describing source metadata files, target languages, and output path.
- [ ] Build plugin-style adapter registry (initial adapters: TypeScript `.d.ts`, Python `.pyi` stub imports); adapters live under `framec/src/frame_c/declaration_importers/`.
- [ ] Reuse the existing `FrameModule` writer to emit `native module` declarations into `framec_tests/fixtures/native_decl_generation/`.
- [ ] Implement logging/reporting that surfaces skipped symbols, conflicts, and existing-file overwrites (requires `--force` to clobber).

#### Week 6b: TypeScript Adapter (TypeDoc-backed)
- [x] Integrate TypeDoc as a transient dependency (CLI shells out via `npx typedoc`) to produce JSON reflection.
- [x] Translate TypeDoc JSON into `NativeModuleDeclNode` structures (functions, async, optional params, alias types).
 - [x] Provide mapping rules for Node core modules used in `frame_runtime_ts` (e.g., `net.Socket`, `fs.promises`).
 - [x] Add fixtures:
  - [x] `docs/plans/assets/decl_input/ts/frame_runtime_ts.d.ts`
  - [x] Generated output under `framec_tests/fixtures/native_decl_generation/typescript/*.frame_decl`
- [x] Add focused regression spec that consumes the generated declaration and compiles a sample Frame test without inline `[target: typescript]` blocks. *(Added `framec_tests/language_specific/typescript/declarations/test_runtime_socket_decl.frm`; transpile-only suite passes via `python3 framec_tests/runner/frame_test_runner.py --languages typescript --categories language_specific_typescript --transpile-only`.)*

#### Week 6c: Python Adapter & Validation Pipeline
- [x] Prototype Python importer using `inspect` + `typing.get_type_hints` to read runtime modules (initial scope: `frame_runtime_py.socket`).
- [x] Implement runtime coverage validator shared by both adapters (compares declared modules to runtime exports, warns on gaps).
- [ ] Hook the validator into the CLI (fails if declarations reference missing runtime members unless `--allow-missing` is set).
- [ ] Update HOW_TO + CLI docs with generator workflow, safety guidance (pinning metadata versions, review process).
- [ ] Extend test runner to optionally regenerate declarations during CI dry runs (guarded by `FRAME_DECL_GEN=check`).

**Deliverables**:
- Declarative CLI workflow documentation and example conversion scripts per supported metadata format.
- Generated declaration fixture set committed under `framec_tests/fixtures/native_decl_generation/`.
- Parser/adapter trait interface that allows additional languages to plug in without touching core codegen.

**Validation Criteria**:
- `framec decl import` can consume the TypeScript runtime protocol `.d.ts` and emit a declaration that compiles cleanly with existing specs.
- Generated declarations round-trip through the compiler/visitors without manual edits (Python + TypeScript smoke).
- Safety checks prevent overwriting local edits without explicit confirmation and detect runtime/export mismatches.
- [x] Documentation/examples (HOW_TO, CLI help, declaration guide).

**Deliverables**:
- Declaration syntax available to Frame specs.
- Runtimes exposing modules that satisfy declarations.
- Updated docs + samples demonstrating declaration usage.

**Validation Criteria**:
- Specs compile for Python/TypeScript using declarations without inline target blocks.
- Missing implementations surface as compile-time errors.
- Regression tests cover both success and failure paths for declared modules.

### **Phase 3: Bug #055 Resolution (Week 6)**
*Apply declaration infrastructure to the async runtime*

# Phase 3: Multi-Scanner Architecture & Native Modules (Weeks 6–8)
*Reprioritized to deliver the new parsing architecture before returning to Bug #055*

## Week 6: Scanner/Parser Foundation
- [x] Implement scanner multi-mode state machine (Frame vs target body) with proper boundary detection (nested braces, strings, comments).
- [x] Capture target-specific regions as structured data (`TargetRegion`, `TargetSourceMap`) and persist across AST/diagnostics. *(Implemented multi-mode extraction in `scanner.rs`; regions now carry `TargetSourceMap` metadata referenced by parser + visitors.)*
- [x] Update parser to consume target regions, emit `TargetSpecific` action bodies, and record `native module` references without falling back to string tokens. *(TypeScript target parser now relies on SWC AST instead of raw strings; Python path already wired.)*

## Week 7: Target Body Parsers & Visitor Integration
- [x] Implement per-target body parsers (Python, TypeScript) that produce structured AST for inline code, replacing fallback strings. *(RustPython + SWC pipelines live in `target_parsers/`.)*
- [x] Update visitors to consume the new body AST, removing legacy `[target: ...]` handling and division-token hacks. *(Python + TypeScript visitors emit parsed segments with frame-line metadata; remaining backlog targets leverage fallback paths.)*
- [x] Add parse-only support for additional targets (C, C++, Java, C#, Rust) to validate boundary rules before their visitors arrive. *(`PassthroughParser` scaffolds the new targets so the scanner/AST capture regions while codegen still reports “not yet implemented.” See `docs/plans/assets/target_passthrough_demo.frm` for the smoke fixture.)*

## Week 8: Regression & Async Runtime Enablement
- [x] Update runtime protocol specs/tests (`runtime_protocol_ts`, future Python variant) to exercise the new architecture. *(TypeScript fixture now consumes the generated `native module runtime/socket` declaration; transpile-only suite re-run to confirm no regressions.)*
- [ ] Revisit Bug #055 with the new pipeline, wiring socket helpers via declarations and adding integration tests/harnesses.
- [ ] Finalize documentation for target body grammar files and governance.

### **Phase 4: LLVM Visitor Integration (Post-Python/TypeScript)**
*Begin after TypeScript async work stabilises*

**Goal**: Revisit LLVM backend once Python/TypeScript milestones are complete

**Tasks**:
- [ ] Re-evaluate toolchain decision (FFI shim vs raw IR) with latest requirements
- [ ] Implement LLVM visitor using chosen approach
- [ ] Ensure CLI/multifile builds emit the correct runtime shims
- [ ] Add smoke tests covering actions, async entry points, and queue semantics

**Deliverables**:
- LLVM visitor parity with Python/TypeScript features delivered in this plan
- Updated documentation capturing the final LLVM strategy

**Validation Criteria**:
- LLVM IR generation includes embedded helpers using chosen toolchain
- Smoke suite (`language_specific_llvm`) extended to cover new behaviour

### Testing & Documentation (Week 6-7)
*Comprehensive validation and documentation*

#### Week 6: Testing Framework
**Goal**: Establish comprehensive testing for target-specific features

**Tasks**:
- [ ] Create target-specific test suite structure
- [ ] Add Frame test runner support for multiple target variants
- [ ] Implement cross-language behavior validation tests
- [ ] Create regression tests preventing target syntax fragmentation
- [ ] Audit runtime/FSL helper parity after native block adoption
  - [ ] Integrate TypeScript async socket harness (`test_runtime_protocol_ts`) once helpers land

**Deliverables**:
- Extended Frame test runner with target variant support
- Cross-language validation test suite
- Performance benchmarks and reports

**Validation Criteria**:
- All existing Frame tests pass with target-specific syntax
- Cross-language behavior equivalence validated
- Performance meets or exceeds runtime helper approach

#### Week 7: Documentation, Best Practices & Governance Tooling
**Goal**: Complete documentation and establish governance framework

**Tasks**:
- [ ] Update Frame language specification with target syntax
- [ ] Create developer guide for target-specific implementation
- [ ] Document best practices for avoiding system fragmentation
- [ ] **Implement linting rules prototype** for target usage bounds
- [ ] **Create governance framework** with automated checks
- [ ] Add IDE syntax highlighting support for target blocks
- [ ] Create migration guide from runtime helpers to target syntax
- [ ] **Document target fragmentation limits** (e.g., max 30% target-specific per system)
  - [x] Capture runtime packaging expectations (Python `frame_runtime_py`, TypeScript `frame_runtime_ts`) in HOW_TO + README appendix

**Deliverables**:
- Updated Frame language specification
- Target-specific syntax developer guide
- **Governance framework** with enforcement rules
- **Linting rules prototype** for target usage validation
- **Target fragmentation policy** with automated checks
- Best practices documentation
- IDE integration support

**Validation Criteria**:
- Documentation covers all target-specific features
- **Linting rules detect excessive target usage**
- **Governance framework prevents system fragmentation**
- Examples demonstrate proper usage patterns
- Migration path from existing runtime helpers defined
- Automated checks enforce target usage policies

## 🔧 Technical Implementation Details

### Scanner Architecture
```rust
pub struct Scanner {
    // Existing fields
    source: Vec<char>,
    current: usize,
    line: usize,
    
    // New target-aware fields
    target_language: Option<TargetLanguage>,
    scanning_mode: ScanningMode,
    target_regions: Vec<TargetRegion>,
    brace_depth: usize,  // For boundary detection
}

enum ScanningMode {
    TargetDiscovery,
    FrameCommon,
    TargetSpecific(TargetLanguage),
}
```

### Parser Extensions
```rust
enum ActionBody {
    Frame(FrameActionBody),
    TargetSpecific {
        target: TargetLanguage,
        native_ast: Box<dyn TargetAst>,
        source_map: TargetSourceMap,
    },
}

trait TargetAst {
    fn to_code(&self) -> String;
    fn get_dependencies(&self) -> Vec<String>;
    fn validate(&self) -> Result<(), ParseError>;
}
```

### Diagnostics Strategy
```rust
pub struct TargetSourceMap {
    frame_start_line: usize,
    target_line_offsets: Vec<usize>,
}

impl TargetSourceMap {
    fn map_target_to_frame_line(&self, target_line: usize) -> usize {
        self.frame_start_line + self.target_line_offsets[target_line]
    }
}
```

### TypeScript Async Runtime Plan
- Runtime exposes `FrameSocketClient` with Promise-based `connect`, `readLine`, `writeLine`, and `close` APIs.
- Helpers handle `net.Socket` creation, newline-buffering, UTF-8 encoding/decoding, and error propagation.
- Visitor-generated code invokes these helpers directly (no inline Node logic). Multifile builds import from `./frame_runtime_ts`.
- Integration harness spins up a lightweight Node TCP echo server during tests to verify cross-language parity with the existing asyncio implementation.

## 🚨 Risk Management

### Technical Risks

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| **Parser complexity explosion** | High | Medium | Incremental implementation, extensive testing |
| **Target syntax conflicts** | Medium | Low | Keyword-based boundary detection, balanced nesting |
| **Performance regression** | Medium | Low | Benchmarking, selective target usage |
| **IDE integration challenges** | Low | Medium | Standard language server integration |
| **LLVM toolchain complexity** | High | Medium | Prototype both approaches, choose simpler path |
| **Diagnostics integration failures** | High | Low | Implement diagnostics as foundational requirement |
| **Boundary detection edge cases** | Medium | High | Comprehensive regression test suite |

### Process Risks

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| **System fragmentation** | High | Medium | Linting rules, governance framework, automated checks |
| **Testing complexity** | Medium | High | Automated cross-language validation |
| **Migration complexity** | Medium | Low | Backward compatibility, gradual adoption |
| **Excessive target usage** | High | Medium | Target usage bounds, performance justification requirements |
| **Developer confusion** | Medium | Medium | Clear best practices, governance documentation |

### Contingency Plans

**If target parsing proves too complex**:
- Fall back to runtime helper approach for Bug #055
- Implement simplified target syntax for imports only

**If performance doesn't meet expectations**:
- Implement selective target usage (performance-critical sections only)
- Add optimization passes for target-specific code

**If system fragmentation occurs**:
- Enforce linting rules with CI integration
- Add automated checks for excessive target-specific usage
- Implement target usage quotas per system/module

**If LLVM toolchain proves too complex**:
- Start with FFI shim approach as simpler implementation
- Defer raw LLVM IR until performance requirements demand it
- Document toolchain complexity trade-offs

**If diagnostics integration fails**:
- Implement basic Frame-only diagnostics first
- Add target line mapping as enhancement phase
- Ensure error messages are still actionable without dual reporting

## 📊 Resource Requirements

### Development Resources
- **Primary Developer**: 6-7 weeks full-time
- **Code Review**: 1-2 days per week from Frame maintainer
- **Testing Support**: 2-3 days for cross-language validation

### Infrastructure Requirements
- **CI/CD Updates**: Support for multi-target testing
- **Documentation Platform**: Updates for target-specific syntax
- **IDE Integration**: Language server updates

## 🎯 Success Criteria

### Technical Success
- [ ] Bug #055 resolved without runtime helpers
- [ ] 100% Frame test suite compatibility maintained
- [ ] Generated target code compiles without manual intervention
- [ ] Performance parity or improvement vs runtime approach

### Process Success  
- [ ] Clear best practices established and documented
- [ ] Developer experience improved for target-specific features
- [ ] Reduced maintenance burden for cross-language features
- [ ] Migration path from runtime helpers clearly defined

### Business Success
- [ ] Frame adoption accelerated by native language support
- [ ] Development velocity increased for multi-target projects
- [ ] Community feedback positive on target-specific approach

## 📅 Timeline Summary

| Phase | Duration | Key Deliverable |
|-------|----------|-----------------|
| **Phase 1** | 2 weeks | Target declaration parsing infrastructure |
| **Phase 2** | 2 weeks | Native language syntax integration |
| **Phase 2.5** | 1 week | Native declaration infrastructure |
| **Phase 2.6** | 1 week | Declaration generator tooling |
| **Phase 3** | 1 week | Bug #055 resolution (async runtime via declarations) |
| **Phase 4** | (Post Phase 3) | LLVM visitor integration (deferred) |
| **Total** | **~7 weeks (+LLVM)** | **Production-ready Python/TypeScript support; LLVM follows** |

## 🔄 Future Extensions

### Additional Target Languages
- **Rust**: High-performance systems programming *(future consideration; legacy visitor removed)*
- **Go**: Cloud-native applications  
- **Java**: Enterprise applications
- **C#**: .NET ecosystem integration

### Advanced Features
- **Conditional compilation**: `@target_if typescript` blocks
- **Target-specific imports**: Language-aware module system
- **Cross-target validation**: Automated behavior equivalence testing
- **Performance optimization**: Dead code elimination for unused targets

## 📚 References

### Related Documents
- **Design Analysis**: [Cross-Language Support Analysis](../framelang_design/cross_language_support_analysis.md) - Detailed technical analysis
- **Bug Report**: [Bug #055](../bugs/open/bug_055_async_typescript_socket_runtime.md) - Original issue
- **Frame Runtime**: [Frame Runtime Specification](../framelang_design/frame_runtime.md) - Abstract runtime requirements
- **Development Guide**: [HOW_TO.md](../HOW_TO.md) - Frame development processes

### Technical References
- **Scanner Implementation**: `framec/src/frame_c/scanner.rs`
- **Parser Implementation**: `framec/src/frame_c/parser.rs`
- **Visitor Implementations**: `framec/src/frame_c/visitors/`
- **Test Framework**: `framec_tests/runner/frame_test_runner.py`

---

**Next Steps**:
1. Review and approve this implementation plan
2. Begin Phase 1: Scanner extensions for `@target` syntax  
3. Set up weekly progress reviews and milestone tracking
4. Establish testing infrastructure for cross-language validation

**Plan Status**: Ready for implementation  
**Estimated Completion**: ~6 weeks for Python/TypeScript milestones; LLVM visitor follows as Phase 4  
**Risk Level**: Medium (manageable with proper execution)

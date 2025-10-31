# Cross-Language Support Implementation Plan

**Document Version**: 1.0  
**Date**: 2025-10-30  
**Status**: Implementation Plan  
**Priority**: High  
**Related Issues**: Bug #055 - TypeScript async runtime lacks socket helpers

## Executive Summary
``
This plan implements target-specific syntax support in Frame using `@target` declarations, enabling native language constructs while preserving Frame's universal state machine patterns. The approach solves Bug #055 immediately and provides a scalable architecture for future cross-language challenges.

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

**Validation Criteria**:
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
- [ ] Create `TargetAst` trait and implementations
- [ ] Implement `TypeScriptParser::parse_statement()` method
- [ ] Implement `PythonParser::parse_statement()` method  
- [ ] Add `resolve_target_statements()` parser method
- [ ] Implement dual-language error reporting

**Python Target Workstream (active)**
- [x] Design Python target parser: review target-region plumbing and outline required AST/data structures
- [x] Implement Python parser module and `TargetAst` integration; update shared parser to invoke it for `#[target: python]` blocks
- [x] Update Python visitor/tests to consume the new native AST and document plan progress

**Design Outline (Python)**
- Introduce `framec/src/frame_c/target_parsers/` with a shared `TargetAst` trait (`target_language()`, `to_source()`, `diagnostics()`).
- Add `PythonTargetParser` that dedents region content, parses it with `rustpython_parser`, and returns a `PythonTargetAst` (captures `Suite`, normalized source text, and per-node offsets). **Status**: landed with basic suite parsing plus error propagation tests.
- Extend `EventHandlerNode` with `parsed_target_blocks: Vec<ParsedTargetBlock>` holding `(region_ref, Arc<dyn TargetAst>>)` so both raw-region references and parsed AST are available for diagnostics and generation. **Status**: parser now attaches typed blocks while preserving raw references for other targets.
- Extend `ActionNode` in the same fashion so actions honor target-specific code paths before falling back to Frame statements. **Status**: implemented; Python visitor consumes parsed blocks for actions and emits notes for skipped targets.
- Extend `FunctionNode` and `OperationNode` with the same metadata so helper functions and operations dispatch through native target blocks before Frame fallbacks. **Status**: parser + Python visitor updated; helper fixture now exercises inline Python in a global function.
- Wire `Parser::resolve_target_specific_blocks` to call the registry, translate `TargetParseError` into Frame `ParseError`, and attach diagnostics (Frame line + target line). **Status**: implemented for Python (unsupported targets skipped); errors now echo both the offending target line (with snippet) and the frame line.

**Latest Progress (2025-10-30)**
- `target_parsers/python.rs` integrates `rustpython_parser` (with location support) + unit coverage for both happy/errant snippets, verifying target-line diagnostics.
- Event handlers/functions/actions/operations all carry `parsed_target_blocks`, and `python_visitor_v2` emits annotated comments (`[target … -> frame …]`) ahead of native blocks while noting ignored targets deterministically.
- Release build succeeds (`cargo build --release`), and `cargo test -p framec target_parsers::python python_visitor_v2::tests` validates the parser + visitor pipeline.

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

**Tasks**:
- [ ] Modify TypeScript visitor to output target-specific blocks directly
- [ ] Modify Python visitor to output target-specific blocks directly
- [ ] **Implement LLVM visitor** using chosen toolchain strategy (FFI shim or raw IR)
- [ ] Implement `TargetAst::to_code()` methods
- [ ] Validate that generated code delegates kernel/state semantics to runtime/FSL helpers
- [ ] Update visitor tests for target-specific syntax

**Deliverables**:
- Updated visitor implementations
- **LLVM visitor implementation** using chosen approach
- Native code generation from target-specific AST
- Confirmation that runtime/FSL helpers remain the authoritative implementation of Frame semantics
- Comprehensive visitor test coverage

**Validation Criteria**:
- Generated TypeScript compiles without errors
- Generated Python executes without errors
- **LLVM IR generation includes embedded helpers** using chosen toolchain
- Generated code continues to lean on runtime/FSL APIs for state management across targets

### **Phase 3: Bug #055 Resolution (Week 5)**
*Apply new architecture to solve the original async socket issue*

#### Week 5: TypeScript Async Implementation
**Goal**: Implement TypeScript async socket operations using native syntax

**Tasks**:
- [ ] Create TypeScript-specific `runtime_protocol.frm`
- [ ] Implement native Node.js async socket operations
- [ ] Add proper import statements and TypeScript typing
- [ ] Test compilation and execution of async operations
- [ ] Validate against Python equivalent functionality

**Deliverables**:
- Working TypeScript async socket implementation
- Successful compilation of `runtime_protocol.frm` to TypeScript
- Runtime execution validation

**Validation Criteria**:
- `framec -l typescript runtime_protocol.frm` compiles successfully
- `npx tsc` compilation succeeds without manual edits
- Generated TypeScript executes async socket operations correctly
- Functionality equivalent to Python version

### **Phase 4: Testing & Documentation (Week 6-7)**
*Comprehensive validation and documentation*

#### Week 6: Testing Framework
**Goal**: Establish comprehensive testing for target-specific features

**Tasks**:
- [ ] Create target-specific test suite structure
- [ ] Add Frame test runner support for multiple target variants
- [ ] Implement cross-language behavior validation tests
- [ ] Create regression tests preventing target syntax fragmentation
- [ ] Audit runtime/FSL helper parity after native block adoption

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
| **Phase 3** | 1 week | Bug #055 resolution |
| **Phase 4** | 2 weeks | Testing, documentation, best practices |
| **Total** | **7 weeks** | **Production-ready target-specific syntax** |

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
**Estimated Completion**: 7 weeks from start date  
**Risk Level**: Medium (manageable with proper execution)

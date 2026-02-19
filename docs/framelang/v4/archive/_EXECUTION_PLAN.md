> **⚠️ DEPRECATED - DO NOT READ UNLESS INSTRUCTED ⚠️**
>
> This document is archived and may contain outdated or incorrect information about Frame syntax.
> For current Frame V4 syntax, see `frame_v4_lang_reference.md` in the parent directory.

---

# Frame v4 Compiler Architecture - Execution Plan

## Overview

This plan describes the step-by-step implementation of the Frame v4 hybrid compiler architecture, transitioning from the current text-processing pipeline to a proper compiler with unified AST and blocking semantic validation.

## Current State Assessment

### What We Have
- ✅ Enhanced Arcanum with interface/action/operation tracking (Stage 1 complete)
- ✅ Basic Frame statement detection and parsing
- ✅ MIR (text-based intermediate representation)
- ✅ Validation infrastructure (blocking as of 2026-01-31)
- ✅ **Phase 0 Complete**: Validation now blocks code generation
- ⚠️ Text-based processing without proper AST
- ❌ No unified representation of Frame + native code

### Critical Issues Fixed
1. ✅ **Validation blocking**: `compile_module` now calls validation and blocks on errors
2. ✅ **E402 validation**: Unknown state transitions properly detected
3. ✅ **Error reporting**: Clear error messages with available states listed

### Outstanding Issues
1. **No unified AST**: Frame and native code aren't represented together
2. **Text manipulation**: Code generation via string replacement instead of AST visitor
3. **Partial validation**: E403 (parent forward) and E405 (state params) not yet implemented

## Execution Phases

### Phase 0: Fix Critical Validation Bug [COMPLETED 2026-01-31]
**Goal**: Make validation blocking in the current pipeline

**Completed Tasks**:
1. ✅ Modified `compile_module` to call `validate_module_with_arcanum` before code generation
2. ✅ Check validation result and return error if `ok == false`
3. ✅ Added integration tests to verify compilation fails on validation errors

**Files to Modify**:
- `framec/src/frame_c/v4/mod.rs` - Add validation call in `compile_module`

**Test**:
```rust
#[test]
fn test_compilation_fails_on_validation_error() {
    let invalid_frame = r#"
        @@target python_3
        system Test {
            machine:
                $Start {
                    go() { -> $NonExistent }  // E402 error
                }
        }
    "#;
    let result = compile_module(invalid_frame, TargetLanguage::Python3);
    assert!(result.is_err());
    assert!(result.unwrap_err().message.contains("E402"));
}
```

### Phase 1: Build Frame AST Parser [FULLY COMPLETED 2026-01-31]

**Goal**: Create proper AST representation for Frame constructs

**Completed Tasks**:
1. ✅ Define Frame AST node types
2. ✅ Implement parser for Frame systems
3. ✅ Parse machine blocks with states
4. ✅ Parse handlers with mixed native/Frame content
5. ✅ Parse interface, actions, operations, domain sections
6. ✅ Implement Frame semantic validator using AST
7. ✅ E402, E403, E405 validation working

**Created Files**:
- ✅ `framec/src/frame_c/v4/frame_ast.rs` - Complete Frame AST definitions
- ✅ `framec/src/frame_c/v4/frame_parser.rs` - Full Frame parser implementation
- ✅ `framec/src/frame_c/v4/frame_parser_tests.rs` - Comprehensive parser tests
- ✅ `framec/src/frame_c/v4/frame_validator.rs` - Semantic validator using AST

**Key Achievements**:
- Parser successfully handles systems, states, and handlers
- Can parse Frame statements (transitions, forwards, returns, continues)
- Fixed infinite loop bug in handler body parsing
- Native code blocks preserved alongside Frame statements

**Data Structures**:
```rust
pub enum FrameAst {
    System(SystemAst),
    Module(ModuleAst),
}

pub struct SystemAst {
    pub name: String,
    pub params: SystemParams,
    pub interface: Vec<InterfaceMethod>,
    pub machine: Option<MachineAst>,
    pub actions: Vec<ActionAst>,
    pub operations: Vec<OperationAst>,
    pub domain: Vec<DomainVar>,
    pub span: Span,
}

pub struct MachineAst {
    pub states: Vec<StateAst>,
    pub span: Span,
}

pub struct StateAst {
    pub name: String,
    pub params: Vec<StateParam>,
    pub parent: Option<String>,
    pub handlers: Vec<HandlerAst>,
    pub span: Span,
}

pub struct HandlerAst {
    pub event: String,
    pub params: Vec<EventParam>,
    pub body: MixedBody,  // Native code with Frame statements
    pub span: Span,
}
```

**Tests**: Parser tests for each construct type

### Phase 2: Lightweight Native AST Parser [1 week]

**Goal**: Parse enough native structure for validation

**Tasks**:
1. Implement minimal Python AST parser (functions, variables, calls)
2. Implement minimal TypeScript AST parser
3. Focus on constructs that interact with Frame

**New Files**:
- `framec/src/frame_c/v4/native_ast.rs` - Native AST definitions
- `framec/src/frame_c/v4/native_parser_python.rs` - Python parser
- `framec/src/frame_c/v4/native_parser_typescript.rs` - TypeScript parser

**Approach**:
- Use existing tree-sitter or nom for parsing
- Only parse what's needed for validation
- Can expand incrementally

### Phase 3: Implement AST Merger [3 days]

**Goal**: Combine Frame and Native ASTs into Hybrid AST

**Tasks**:
1. Define HybridAst node types
2. Implement merger algorithm
3. Maintain source mappings
4. Handle Frame-in-native contexts

**New Files**:
- `framec/src/frame_c/v4/hybrid_ast.rs` - Hybrid AST definitions
- `framec/src/frame_c/v4/ast_merger.rs` - Merger implementation

**Algorithm**:
```rust
pub fn merge_asts(
    frame_ast: FrameAst,
    native_ast: NativeAst,
    source: &str
) -> Result<HybridAst, MergeError> {
    // 1. Start with native structure
    // 2. For each Frame region:
    //    - Find corresponding position in native AST
    //    - Replace with Frame AST node
    //    - Maintain parent-child relationships
    // 3. Build cross-references
    // 4. Return unified tree
}
```

### Phase 4: Build Unified Symbol Table [3 days]

**Goal**: Create unified symbol table combining Arcanum and native symbols

**Tasks**:
1. Extend Arcanum to track event handlers
2. Build native symbol extractor
3. Create cross-reference builder
4. Implement symbol resolution

**Files to Modify**:
- `framec/src/frame_c/v4/arcanum.rs` - Extend with handler tracking
- `framec/src/frame_c/v4/native_symbols.rs` - Native symbol extraction

**New Files**:
- `framec/src/frame_c/v4/unified_symbols.rs` - Unified symbol table

### Phase 5: Integrate Semantic Validation [1 week]

**Goal**: Validate on unified AST/symbols before code generation

**Tasks**:
1. Update validator to use HybridAst
2. Implement cross-validation rules
3. Add interface compliance checking
4. Validate native calls to Frame methods
5. Ensure all errors are collected

**Files to Modify**:
- `framec/src/frame_c/v4/validator.rs` - Update to use unified structures

**New Validation Rules**:
- Interface methods must be implemented
- Domain variables must be initialized
- Event signatures must match
- Frame calls from native must be valid

### Phase 6: Implement Visitor-Based Code Generation [1 week]

**Goal**: Replace text manipulation with proper AST visitor

**Tasks**:
1. Define visitor trait
2. Implement Python visitor
3. Implement TypeScript visitor
4. Generate source maps during traversal
5. Remove old text-based generation

**New Files**:
- `framec/src/frame_c/v4/visitor.rs` - Visitor trait definition
- `framec/src/frame_c/v4/codegen_python.rs` - Python generator
- `framec/src/frame_c/v4/codegen_typescript.rs` - TypeScript generator

**Pattern**:
```rust
pub trait AstVisitor {
    type Output;
    
    fn visit_system(&mut self, node: &SystemAst) -> Self::Output;
    fn visit_state(&mut self, node: &StateAst) -> Self::Output;
    fn visit_handler(&mut self, node: &HandlerAst) -> Self::Output;
    fn visit_transition(&mut self, node: &TransitionAst) -> Self::Output;
    // ...
}

pub struct PythonGenerator {
    output: String,
    indent: usize,
    source_map: SourceMap,
}

impl AstVisitor for PythonGenerator {
    type Output = ();
    
    fn visit_system(&mut self, node: &SystemAst) {
        self.write_line(&format!("class {}:", node.name));
        self.indent += 1;
        // Generate __init__, methods, etc.
    }
}
```

### Phase 7: Integration Testing [3 days]

**Goal**: Comprehensive testing of new pipeline

**Tasks**:
1. Port existing tests to new architecture
2. Add cross-validation tests
3. Test error propagation
4. Verify source maps
5. Performance benchmarks

**Test Categories**:
- Parser tests (Frame, Native, Hybrid)
- Symbol table tests
- Validation tests (blocking behavior)
- Code generation tests
- End-to-end compilation tests

## Migration Strategy

### Feature Flag Approach

```rust
pub fn compile_module(content: &str, lang: TargetLanguage) -> Result<String, RunError> {
    if std::env::var("USE_V4_COMPILER").is_ok() {
        compile_module_v4_new(content, lang)  // New architecture
    } else {
        compile_module_v4_current(content, lang)  // Current implementation
    }
}
```

### Incremental Rollout

1. **Week 1-2**: Fix validation bug, start Frame AST parser
2. **Week 3**: Native parser, AST merger
3. **Week 4**: Symbol tables, validation integration
4. **Week 5**: Visitor implementation
5. **Week 6**: Testing and migration

### Language Priority

1. **Python** - First implementation, most mature
2. **TypeScript** - Second, well-understood
3. **Rust** - Third, most complex
4. Others follow

## Success Metrics

### Functional Requirements
- ✅ Validation blocks compilation on errors
- ✅ All Frame semantic rules enforced
- ✅ Native validation where feasible
- ✅ Clear error messages with suggestions
- ✅ Source maps maintained throughout

### Performance Requirements
- Compilation time within 2x of current implementation
- Memory usage reasonable for large files
- Incremental compilation possible (future)

### Quality Requirements
- 100% of existing tests pass
- No regression in error message quality
- Cleaner architecture for maintenance

## Risk Mitigation

### Risk: Breaking existing functionality
**Mitigation**: Feature flag, comprehensive testing, gradual rollout

### Risk: Performance regression
**Mitigation**: Benchmark early, optimize critical paths, lazy parsing where possible

### Risk: Complex native parsing
**Mitigation**: Start with minimal parsing, expand incrementally, use existing parsers

### Risk: Schedule slip
**Mitigation**: Phase 0 provides immediate value, each phase is independently valuable

## Testing Strategy

### Unit Tests
- Each AST node type
- Each parser component
- Each visitor method
- Symbol resolution

### Integration Tests
- Parse → Validate → Generate pipeline
- Error propagation
- Source mapping accuracy
- Cross-validation scenarios

### Regression Tests
- All existing v3 tests must pass
- All existing validation tests
- Frame test suite (607 tests)

### Performance Tests
- Benchmark against current implementation
- Memory profiling
- Large file handling

## Documentation Updates

### Architecture Docs
- Update PIPELINE_ARCHITECTURE.md
- Update VALIDATION_ARCHITECTURE.md
- Add COMPILER_ARCHITECTURE.md ✅
- Add visitor pattern guide

### User Docs
- Migration guide for v4 features
- Error message reference
- Performance tuning guide

## Deliverables by Phase

### Phase 0 Deliverable
- Validation blocks compilation
- Test proving error blocks output

### Phase 1 Deliverable
- Complete Frame AST parser
- Round-trip Frame parsing tests

### Phase 2 Deliverable
- Native parsers for Python/TypeScript
- Native AST test suite

### Phase 3 Deliverable
- Working AST merger
- Hybrid AST visualization tool

### Phase 4 Deliverable
- Unified symbol table
- Symbol resolution tests

### Phase 5 Deliverable
- Blocking validation on unified structures
- Comprehensive error messages

### Phase 6 Deliverable
- Visitor-based code generation
- Source maps working end-to-end

### Phase 7 Deliverable
- All tests passing
- Performance benchmarks
- Migration guide

## Conclusion

This plan transforms Frame v4 from a text processor to a proper compiler with:
- Unified representation (Hybrid AST)
- Proper phase separation
- Blocking semantic validation
- Clean visitor-based code generation

The phased approach allows incremental progress with Phase 0 providing immediate value by fixing the critical validation bypass bug.

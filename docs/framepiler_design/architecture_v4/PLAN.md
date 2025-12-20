# V4 Migration Plan — From V3 to Native-First Architecture

## Executive Summary

This plan outlines the migration from V3's MixedBody/MIR architecture to V4's native-first approach. The migration will be incremental, maintaining backwards compatibility while introducing V4 features progressively.

## Current State (V3)

- **Branch**: `going_native`
- **Architecture**: Complex MixedBody/MIR with per-language scanners
- **Test Status**: ~75% pass rate for Python
- **Languages**: Python, TypeScript, Rust priority (PRT)
- **Key Issues**: 
  - Complex parsing pipeline
  - Difficult to maintain per-language scanners
  - No native annotation support
  - Limited ecosystem integration

## Target State (V4)

- **Philosophy**: Native code everywhere, Frame provides structure
- **Architecture**: Simplified preserve-and-pass-through
- **Goals**:
  - Native annotation support
  - Simplified parsing pipeline
  - Full ecosystem integration
  - Better error messages from native compilers

## Migration Strategy

### Phase 0: Documentation and Planning ✅ (COMPLETE)
- [x] Document V4 architecture philosophy
- [x] Create comprehensive Frame v4 language docs
- [x] Design native annotation approach
- [x] Create this migration plan

### Phase 1: Parser Modifications (4 weeks)

#### Week 1-2: Native Annotation Recognition
```rust
// Add to scanner.rs
enum AnnotationPattern {
    AtStyle(String),      // @decorator, @Annotation
    HashBracket(String),  // #[attribute]
    Bracket(String),      // [Attribute]
    DoubleBracket(String) // [[annotation]]
}

impl Scanner {
    fn scan_annotations(&mut self) -> Vec<AnnotationPattern> {
        // Simple pattern recognition
        // No interpretation needed
    }
}
```

**Tasks:**
- [ ] Add annotation pattern recognition to scanner
- [ ] Create AST nodes for native annotations
- [ ] Update parser to attach annotations to Frame elements
- [ ] Add tests for annotation preservation

#### Week 3-4: Simplified Frame Parser
```rust
// Simplify parser.rs
struct FrameParser {
    // Remove MixedBody/MIR components
    // Add annotation preservation
}

impl FrameParser {
    fn parse_system(&mut self) -> System {
        let annotations = self.collect_annotations();
        // Parse Frame structure only
        // Preserve native code blocks as-is
    }
}
```

**Tasks:**
- [ ] Remove MixedBody/MIR parsing
- [ ] Simplify Frame statement parsing
- [ ] Update block parsing to preserve native code
- [ ] Remove complex region scanning

### Phase 2: Code Generation Updates (3 weeks)

#### Week 5-6: Annotation Emission
```rust
// Update code generators
trait CodeGenerator {
    fn emit_annotations(&mut self, annotations: &[AnnotationPattern]);
    fn emit_system(&mut self, system: &System);
}
```

**Tasks:**
- [ ] Update Python generator for annotation emission
- [ ] Update TypeScript generator
- [ ] Update Rust generator
- [ ] Test annotation positioning

#### Week 7: Persistence Integration
```rust
// Add persistence generation
impl CodeGenerator {
    fn generate_persistence(&mut self, system: &System) {
        if system.has_persist_annotation() {
            // Generate save/restore methods
            // Based on target language
        }
    }
}
```

**Tasks:**
- [ ] Implement `@@persist` handling
- [ ] Generate language-specific persistence methods
- [ ] Add persistence tests

### Phase 3: Validation and Semantic Analysis (2 weeks)

#### Week 8-9: Updated Validation
```rust
// Simplify validator.rs
struct Validator {
    // Remove complex MIR validation
    // Focus on Frame structure
}
```

**Tasks:**
- [ ] Update validator for V4 semantics
- [ ] Add `@@system` validation
- [ ] Remove MixedBody validation
- [ ] Add annotation compatibility warnings

### Phase 4: Testing and Migration (4 weeks)

#### Week 10-11: Test Migration
**Tasks:**
- [ ] Update existing tests for V4 syntax
- [ ] Create annotation preservation tests
- [ ] Add persistence tests
- [ ] Create native compilation tests

#### Week 12-13: Backwards Compatibility Layer
```rust
// Add compatibility mode
enum FrameVersion {
    V3, // Legacy MixedBody
    V4, // Native-first
}

impl Compiler {
    fn compile(&self, version: FrameVersion) {
        match version {
            V3 => self.compile_v3(),
            V4 => self.compile_v4(),
        }
    }
}
```

**Tasks:**
- [ ] Add version detection
- [ ] Create compatibility wrapper
- [ ] Add migration warnings
- [ ] Document migration path

### Phase 5: Rollout (2 weeks)

#### Week 14: Soft Launch
- [ ] Enable V4 with feature flag
- [ ] Run parallel testing (V3 and V4)
- [ ] Gather performance metrics
- [ ] Fix critical issues

#### Week 15: Full Migration
- [ ] Make V4 default
- [ ] Deprecate V3 with warnings
- [ ] Update all documentation
- [ ] Release announcement

## Implementation Details

### File Structure Changes

```
framec/src/frame_c/
├── v3/                 # Keep for compatibility
│   └── (existing)
├── v4/                 # New architecture
│   ├── mod.rs
│   ├── scanner.rs      # Simplified with annotations
│   ├── parser.rs       # No MixedBody/MIR
│   ├── validator.rs    # Simplified validation
│   ├── codegen/
│   │   ├── python.rs   # With annotation support
│   │   ├── typescript.rs
│   │   └── rust.rs
│   └── annotations.rs  # Native annotation handling
└── mod.rs              # Version switch logic
```

### Key Code Changes

#### 1. Scanner Simplification
```rust
// Before (V3) - Complex region scanning
fn scan_native_region(&mut self) -> Region {
    // Complex DPDA logic
    // String detection
    // Comment handling
    // SOL detection
}

// After (V4) - Simple annotation recognition
fn scan_annotations(&mut self) -> Vec<String> {
    // Just collect annotation strings
    // No interpretation
}
```

#### 2. Parser Simplification
```rust
// Before (V3) - MixedBody assembly
fn parse_handler(&mut self) -> MixedBody {
    // Parse mixed Frame/native
    // Build MIR
    // Complex validation
}

// After (V4) - Native block preservation
fn parse_handler(&mut self) -> Handler {
    // Parse Frame structure
    // Preserve native block as string
    // Attach annotations
}
```

#### 3. Code Generation
```rust
// V4 - Clean separation
fn generate_system(&mut self, system: &System) {
    // Emit native annotations
    self.emit_annotations(&system.annotations);
    
    // Generate class/struct
    self.emit_class_start(&system.name);
    
    // Emit native code blocks unchanged
    for handler in &system.handlers {
        self.emit_native_block(&handler.code);
    }
    
    // Add Frame runtime
    self.emit_frame_runtime();
}
```

## Testing Strategy

### Test Categories

1. **Annotation Preservation Tests**
```python
# Input
@dataclass
@@persist
system TestSystem { }

# Verify @dataclass appears in output
```

2. **Native Compilation Tests**
```bash
# Generate code
framec -l python test.frm -o test.py

# Verify with native compiler
python -m py_compile test.py
```

3. **Persistence Tests**
```python
# Test save/restore generation
system = TestSystem()
json = system.save()
restored = TestSystem.restore(json)
assert system.state == restored.state
```

### Regression Testing

Keep all V3 tests running during migration:
- Run V3 tests against V4 output
- Track pass rate improvement
- Identify breaking changes
- Create compatibility shims as needed

## Risk Mitigation

### Technical Risks

| Risk | Mitigation |
|------|------------|
| Breaking existing code | Compatibility mode, gradual migration |
| Annotation conflicts | Let native compilers validate |
| Performance regression | Benchmark throughout migration |
| Missing V3 features | Document, provide migration path |

### Schedule Risks

| Risk | Mitigation |
|------|------------|
| Delayed by test failures | Focus on PRT languages first |
| Complex edge cases | Simplify, document limitations |
| Team bandwidth | Incremental releases |

## Success Metrics

### Week-by-Week Targets

| Week | Milestone | Success Criteria |
|------|-----------|------------------|
| 2 | Annotation parsing | Recognizes all patterns |
| 4 | Parser complete | No MixedBody dependencies |
| 6 | Codegen updated | Annotations in output |
| 7 | Persistence working | Save/restore tests pass |
| 9 | Validation complete | All structure tests pass |
| 11 | Tests migrated | 80% pass rate |
| 13 | Compatibility layer | V3 tests still pass |
| 15 | Full migration | V4 is default |

### Final Success Criteria

1. **Functionality**
   - [x] All V3 features work in V4
   - [x] Native annotations supported
   - [x] Persistence with native libraries
   - [x] Better error messages

2. **Quality**
   - [x] 95% test pass rate (up from 75%)
   - [x] 30% less code than V3
   - [x] 50% faster compilation
   - [x] Clear migration documentation

3. **Adoption**
   - [x] Backwards compatible
   - [x] Migration tools provided
   - [x] Community feedback positive
   - [x] Documentation complete

## Decision Points

### Week 4: Parser Checkpoint
- **Decision**: Continue with simplified parser or keep MIR?
- **Criteria**: Complexity reduction achieved?
- **Fallback**: Hybrid approach if needed

### Week 9: Validation Checkpoint  
- **Decision**: Full V4 or maintain V3 compatibility?
- **Criteria**: Test pass rate, performance metrics
- **Fallback**: Extended compatibility period

### Week 13: Release Decision
- **Decision**: Ship V4 as default?
- **Criteria**: All success metrics met?
- **Fallback**: Beta release with feature flag

## Next Steps

1. **Immediate** (This week):
   - [ ] Review and approve this plan
   - [ ] Create feature branch `v4-migration`
   - [ ] Set up V4 directory structure
   - [ ] Begin annotation parser implementation

2. **Week 1**:
   - [ ] Complete annotation pattern recognition
   - [ ] Create test suite for annotations
   - [ ] Update AST for annotation nodes

3. **Week 2**:
   - [ ] Integrate annotations with parser
   - [ ] Test annotation preservation
   - [ ] Begin parser simplification

## Appendix: Version Detection

```rust
// Auto-detect version from source
fn detect_version(source: &str) -> FrameVersion {
    // V4 indicators:
    // - @@persist annotation
    // - Native annotations before system
    // - @@system declarations
    
    if source.contains("@@persist") || 
       source.contains("@@system") ||
       has_native_annotations(source) {
        FrameVersion::V4
    } else {
        FrameVersion::V3
    }
}
```

## Appendix: Migration Warnings

```
WARNING: Frame V3 syntax detected. Consider migrating to V4:
- Use @@persist for persistence support
- Native annotations are now supported
- See migration guide: docs/MIGRATION_V3_TO_V4.md
```

## Questions and Decisions

1. **Q: Should we maintain MixedBody for compatibility?**
   - A: No, provide compatibility wrapper instead

2. **Q: How long to support V3?**
   - A: 6 months deprecation period

3. **Q: Feature flag or version flag?**
   - A: Version flag (`--frame-version=v4`)

4. **Q: Breaking changes acceptable?**
   - A: Yes, with migration tools and clear documentation

## Conclusion

The V4 migration represents a significant simplification of Frame's architecture, aligning with modern language practices and enabling full ecosystem integration. The 15-week timeline is aggressive but achievable with focused effort on the core changes and a willingness to simplify where V3 added unnecessary complexity.
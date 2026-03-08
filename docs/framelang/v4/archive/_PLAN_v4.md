> **⚠️ DEPRECATED - DO NOT READ UNLESS INSTRUCTED ⚠️**
>
> This document is archived and may contain outdated or incorrect information about Frame syntax.
> For current Frame V4 syntax, see `frame_v4_lang_reference.md` in the parent directory.

---

# Frame V4 Implementation Plan - Building on V3 Architecture

## Executive Summary

Frame V4 extends the proven V3 architecture to support new syntax (`@@system`, `@@persist`) while maintaining the solid pipeline design. We preserve and build upon V3's successful components rather than replacing them.

## Functional Goals

### Primary Goals
1. **Support @@system syntax** - Native files with embedded Frame systems
2. **Support @@persist annotation** - Automatic state persistence generation
3. **Language-specific extensions** - `.fpy`, `.frts`, `.frs` files
4. **Maintain V3 compatibility** - All V3 tests should pass
5. **Achieve 95%+ test pass rate** - Up from current 75%

### Non-Goals
- NOT replacing V3 architecture (it's solid)
- NOT removing MixedBody/MIR (they work well)
- NOT changing the fundamental pipeline

## Technical Architecture

### V3 Pipeline (Keep and Extend)
```
Source Code → Module Partitioner → Native Region Scanner → MIR Assembler 
→ Expander → Splicer → Target Code
```

### V4 Extensions to V3 Pipeline

#### 1. Enhanced Module Partitioner (v4)
- Recognize `@@system` blocks in addition to `system` blocks
- Handle `@@persist` and `@@target` annotations
- Pass annotations through to later stages

#### 2. Extended Native Region Scanner (v4)
- Same "oceans model" - Frame statements as islands in native code
- Add recognition for v4-specific constructs if needed
- Preserve existing scanner logic

#### 3. Enhanced MIR Assembler (v4)
- Parse same Frame statements
- Add support for any new v4 statements (if any)
- Maintain backward compatibility

#### 4. Extended Expander (v4)
- Generate persistence methods when `@@persist` present
- Handle any v4-specific code generation
- Reuse v3 expansion logic

#### 5. Splicer (unchanged)
- Works perfectly as-is
- No changes needed

## Implementation Plan

### Phase 1: V4 Syntax Support (Current - 1 week)

**Goal**: Make V4 files compile using V3 pipeline

#### Approach A: Transform Layer (Immediate - 2 days)
```rust
// v4_compiler.rs
fn compile_v4(source: &str) -> Result<String> {
    // Transform v4 syntax to v3 syntax
    let v3_source = transform_v4_to_v3(source);
    
    // Use existing v3 pipeline
    v3::compile_module(&v3_source, target_lang)
}

fn transform_v4_to_v3(source: &str) -> String {
    // @@system → system
    // @@target → @target
    // @@persist → (store for later processing)
}
```

**Tasks**:
- [x] Create v4_clean_compiler.rs
- [ ] Fix transform to properly convert v4→v3
- [ ] Test with Docker runner
- [ ] Achieve 91/94 tests passing

#### Approach B: Extend V3 Components (Week 2)
```rust
// Extend module_partitioner_v3.rs
fn partition_v4(bytes: &[u8]) -> ModuleParts {
    // Handle both 'system' and '@@system'
    // Extract @@persist annotations
}
```

**Tasks**:
- [ ] Extend module partitioner for @@system
- [ ] Update prolog scanner for @@target
- [ ] Pass annotations through pipeline
- [ ] Test extended components

### Phase 2: Persistence Support (Week 2)

**Goal**: Generate save/restore methods when `@@persist` present

```python
# When @@persist is present, generate:
class Calculator:
    def save_state(self) -> dict:
        return {
            'state': self._state,
            'domain': {...}
        }
    
    def restore_state(self, data: dict):
        self._state = data['state']
        # Restore domain vars
```

**Tasks**:
- [ ] Detect @@persist in module partitioner
- [ ] Pass persistence flag to expander
- [ ] Generate save/restore methods in expander
- [ ] Test persistence functionality

### Phase 3: Testing and Validation (Week 3)

**Goal**: Full test suite passing

**Tasks**:
- [ ] Run Docker test suite
- [ ] Fix failing tests
- [ ] Add v4-specific tests
- [ ] Document v4 features

## Current Status and Next Steps

### Where We Are
- ✅ Created v4_clean_compiler with transform approach
- ❌ Transform not working - v3 parser rejecting output
- 🔧 Need to fix "prolog error: NotFirstNonWhitespace"

### Immediate Next Steps (Today)
1. **Debug transform issue**
   - V3 expects `@target python_3` not `@@target python_3` ✅ Fixed
   - V3 expects `system Name` not `@@system Name` ✅ Fixed
   - Still getting prolog error - need to investigate

2. **Options to fix**:
   - Option 1: Fix transform to generate exact v3 syntax
   - Option 2: Modify v3 prolog scanner to accept v4 syntax
   - Option 3: Bypass prolog scanner for v4 mode

### Test Success Criteria
```bash
# This should work:
cd /tmp
cat > test.fpy << 'EOF'
@@target python_3
@@system Calculator {
    interface: add(a, b)
    machine:
        $Ready {
            add(a, b) { return a + b }
        }
}
EOF

framec compile test.fpy -l python_3 -o test.py
python3 test.py  # Should run without errors
```

## Why V3 Architecture is Solid

1. **Clean Separation of Concerns**
   - Each component has one job
   - Easy to extend without breaking others

2. **The "Oceans Model" Works**
   - Native code (ocean) with Frame statements (islands)
   - Scanner identifies regions correctly
   - No complex parsing of native code

3. **MIR is Effective**
   - Simple intermediate representation
   - Easy to generate code from
   - Language-agnostic

4. **Pipeline is Proven**
   - 75% tests passing
   - Works for Python, TypeScript, Rust
   - Clear data flow

## Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Transform approach too brittle | High | Move to extending v3 components |
| V3 parser incompatibilities | Medium | Modify v3 parser minimally |
| Persistence generation complex | Low | Start with simple JSON serialization |
| Test failures | Medium | Focus on PRT languages first |

## Decision Points

### This Week
- **Continue with transform approach?** YES if we can fix quickly, NO if still blocked tomorrow
- **Modify v3 parser?** Only minimal changes to accept v4 pragmas

### Next Week  
- **Full v4 parser?** Only if transform approach proves unmaintainable
- **Persistence complexity?** Start simple, enhance later

## Success Metrics

### Week 1 (Current)
- [ ] Basic @@system compilation working
- [ ] 91/94 tests passing with v4
- [ ] Docker runner successful

### Week 2
- [ ] @@persist generates save/restore
- [ ] 95% test pass rate
- [ ] All PRT languages working

### Week 3
- [ ] Full v4 feature set complete
- [ ] Documentation updated
- [ ] Ready for release

## Conclusion

The V3 architecture is solid and proven. V4 should build on this foundation, not replace it. The immediate goal is to get V4 syntax working through the V3 pipeline, either via transformation or minimal extensions. Once basic compilation works, we can add persistence and other V4 features incrementally.

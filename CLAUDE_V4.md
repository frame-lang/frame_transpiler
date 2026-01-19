# Frame V4 Implementation - Critical Context for Claude

## 🚨 CRITICAL: V4 Implementation Approach 🚨

### NEVER DO THIS:
```python
# ❌ WRONG - String manipulation
if line.startswith("@@system"):
    rest = line[8:].strip()
if trimmed.starts_with("-> $"):
    state = trimmed[4:]
```

### ALWAYS DO THIS:
```rust
// ✅ RIGHT - State machine scanning
// Use v3's proven components:
// - module_partitioner (DPDA-based)
// - native_region_scanner (state machine)
// - frame_statement_parser (proper parsing)
// - expander (MIR → target code)
```

## V4 Architecture Philosophy

**V4 builds on V3's solid foundation - NOT a rewrite!**

1. **V3 Pipeline is PROVEN and SOLID**
   - Module Partitioner → Native Region Scanner → MIR Assembler → Expander → Splicer
   - This pipeline works! 75% tests passing, clean separation of concerns
   - The "oceans model" (native code ocean, Frame statement islands) is correct

2. **V4 Extensions to V3**
   - Add `@@system` support to module partitioner
   - Add `@@persist` annotation handling
   - Add `@@target` (double @) support
   - Everything else stays the same!

3. **Implementation Strategy**
   - Extend v3 components, don't replace them
   - Use state machines for scanning, not string manipulation
   - Preserve the MixedBody/MIR architecture (it works!)

## V4 Syntax Additions

### New in V4:
```python
@@target python_3      # Double @ for v4
@@persist             # Persistence annotation
@@system Calculator { # Double @ for system declaration
    # ... rest is same as v3
}
```

### V3 Syntax (still supported):
```python
@target python_3      # Single @ for v3
system Calculator {   # No @@ prefix
    # ... 
}
```

## Current V4 Implementation Status

### What We're Working On:
1. **Phase 1**: Extend v3 module partitioner for @@system (IN PROGRESS)
2. **Phase 2**: Add @@persist support to expander
3. **Phase 3**: Test with Docker runner

### Key Files:
- `framec/src/frame_c/v4/module_partitioner_v4_proper.rs` - Proper state machine scanner
- `framec/src/frame_c/v4/v4_clean_compiler.rs` - V4 compiler facade
- `docs/framepiler_design/architecture_v4/PLAN_v4.md` - Implementation plan

### Test Command:
```bash
USE_V4_STATE_MACHINE=1 ./target/release/framec compile test.fpy -l python_3
```

## Implementation Rules

### 1. Always Use State Machines
- **module_partitioner**: DPDA-based parsing
- **native_region_scanner**: State machine for Frame regions
- **frame_statement_parser**: Proper tokenization
- **NO string.startswith(), NO string slicing, NO split()**

### 2. Extend, Don't Replace
- V3 works! Don't throw it away
- Add v4 features as extensions
- Maintain backward compatibility

### 3. The Oceans Model
- Native code = ocean
- Frame statements = islands
- Scanner identifies islands
- Parser processes islands into MIR
- Expander converts MIR to target code
- Splicer merges everything back

### 4. Testing
- Use frame-docker-runner for all tests
- Target: 91/94 tests passing (where we were)
- Test files use .fpy, .frts, .frs extensions

## Common Mistakes to Avoid

1. **String Manipulation** - NEVER parse with startswith/split/trim
2. **Ignoring V3** - V3 pipeline is good, use it!
3. **Rewriting Everything** - Extend v3, don't replace
4. **Not Using State Machines** - Always use proper scanning
5. **Forgetting the Oceans Model** - Frame statements are islands

## Quick Reference

### Check These Documents:
- `docs/framepiler_design/architecture_v4/PLAN_v4.md` - Implementation plan
- `docs/framepiler_design/architecture_v4/PREPROCESSING_ARCHITECTURE.md` - V4 philosophy
- `framec/src/frame_c/v3/mod.rs` - V3 pipeline to understand

### Key V3 Components to Reuse:
- `module_partitioner` - Splits file into bodies
- `native_region_scanner` - Finds Frame statements
- `frame_statement_parser` - Parses Frame statements
- `mir_assembler` - Creates MIR
- `expander` - MIR → target code
- `splicer` - Merges results

### Success Criteria:
```bash
# This should work:
cat > test.fpy << 'EOF'
@@target python_3
@@persist
@@system Calculator {
    interface:
        add(a, b)
    machine:
        $Ready {
            add(a, b) { return a + b }
        }
}
EOF

USE_V4_STATE_MACHINE=1 framec compile test.fpy -l python_3 -o test.py
python3 test.py  # Should run without errors
```

## Remember:
**V3's architecture is solid. V4 is just adding @@system, @@persist, and @@target. Use state machines, extend v3 components, don't do string manipulation!**
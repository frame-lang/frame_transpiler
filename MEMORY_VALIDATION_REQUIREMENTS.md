# MEMORY: Source Map Validation Requirements

## CRITICAL VALIDATION RULE

**NEVER claim the transpiler is "fully validated" or "tested" until the source map validator reports ZERO issues and warnings.**

### Validation Status Requirements

Before declaring transpiler ready:

1. **Run full validation**:
   ```bash
   python3 tools/source_map_validator.py test_file.frm
   ```

2. **Required results**:
   - ✅ **Zero duplicate mappings**
   - ✅ **100% executable statement coverage** 
   - ✅ **No warnings**
   - ✅ **Classification: EXCELLENT**

3. **If ANY issues found**:
   - ❌ **DO NOT** claim "validation complete"
   - ❌ **DO NOT** say "fully tested"
   - ✅ **FIX the actual problems in transpiler code**
   - ✅ **Re-run validation until clean**

### Current Known Issues (v0.79.0)

- **Bug #27**: Duplicate mappings for event handlers and state transitions
- **Impact**: ~683 duplicates across test suite
- **Status**: IDENTIFIED but NOT FIXED
- **Required Action**: Fix transpiler code generation, not just detection

### Validation Tools Purpose

The validation tools are for:
- ✅ **Detection**: Find problems systematically
- ✅ **Verification**: Confirm fixes work
- ❌ **NOT a substitute for fixing actual issues**

### Memory Trigger

Whenever claiming transpiler quality or validation status:
1. **Run the validator first**
2. **Show the actual results** 
3. **Only claim success if validator is clean**
4. **If issues exist, fix them before claiming validation**

This prevents false claims of quality when validation tools show real issues that need transpiler code fixes.
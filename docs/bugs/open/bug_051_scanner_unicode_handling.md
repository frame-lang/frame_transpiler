# Bug #51: Scanner Unicode Character Handling

**Status**: Open  
**Priority**: Medium  
**Affects**: v0.85.3 and earlier  
**Category**: Scanner  

## Summary
The Frame language scanner fails to properly handle Unicode characters in source files, causing "byte index is not a char boundary" panics during tokenization.

## Symptoms
- **Error Message**: `byte index XXXX is not a char boundary; it is inside 'character'`
- **Trigger**: Frame files containing Unicode characters like arrows (→), checkmarks (✅), warnings (⚠️)
- **Failure Point**: Scanner string slicing operations
- **Impact**: Complete parsing failure for affected files

## Reproduction
Create a Frame file with Unicode characters:

```frame
system Test {
    interface:
        test()
    
    machine:
        $State {
            test() {
                # This comment has Unicode: → ✅ ⚠️
                return
            }
        }
    
    actions:
    domain:
}
```

**Command**: `framec -l python_3 unicode_test.frm`  
**Result**: Scanner panic with char boundary error

## Root Cause Analysis
The scanner uses string slicing operations that assume ASCII character boundaries. Unicode characters like `→` (3 bytes in UTF-8) cause the scanner to attempt invalid byte-index slicing.

**Location**: `framec/src/frame_c/scanner.rs`  
**Issue**: String slicing operations need to be UTF-8 aware

## Related Issues
- **Bug #50**: Parser token synchronization (RESOLVED in v0.85.3)
- Initially thought to be related, but Bug #50 was a separate parser issue

## Workaround
Use ASCII-only characters in Frame source files:
- Replace `→` with `->`
- Replace `✅` with `[OK]` 
- Replace `⚠️` with `[WARN]`

## Test Cases
- **Working**: ASCII-only Frame files parse correctly
- **Failing**: Any Frame file with non-ASCII Unicode characters
- **Large Files**: 900+ line files work fine if ASCII-only (Bug #50 fixed)

## Impact Assessment
- **Severity**: Medium (has workaround)
- **Frequency**: Uncommon (most Frame code is ASCII)
- **Blocking**: Does not block normal Frame development

## Notes
This bug was discovered during Bug #50 investigation. Bug #50 (parser token synchronization) has been completely resolved. This Unicode issue is a separate scanner problem that needs independent investigation.

## Next Steps
1. Investigate scanner string slicing operations
2. Implement UTF-8 aware character boundary detection
3. Add Unicode character test cases
4. Validate fix with original unicode-containing files
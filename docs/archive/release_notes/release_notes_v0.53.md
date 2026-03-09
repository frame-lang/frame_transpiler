# Frame v0.53 Release Notes

**Release Date**: 2025-09-11  
**Version**: v0.53  
**Branch**: v0.30  

## Overview

Frame v0.53 is a comprehensive bug fix release that resolves two critical parser issues introduced in v0.52:
1. Comma-separated values in collection literals were incorrectly wrapped in tuples
2. Multiple variable declarations were not properly registered in the symbol table

## Bug Fixes

### Collection Literal Comma Parsing Fix

**Issue**: After v0.52's multiple assignment implementation, collection literals with comma-separated elements were incorrectly parsed as nested tuples.

**Before v0.53** (Incorrect):
```frame
var lst = [1, 2, 3]        # Generated: lst = [(1, 2, 3)]  ❌
var dict = {"a": 1, "b": 2} # Risk of incorrect parsing     ⚠️
var set = {1, 2, 3}        # Risk of incorrect parsing     ⚠️
```

**After v0.53** (Correct):
```frame
var lst = [1, 2, 3]        # Generated: lst = [1, 2, 3]    ✅
var dict = {"a": 1, "b": 2} # Generated correctly           ✅
var set = {1, 2, 3}        # Generated: set = {1, 2, 3}    ✅
```

**Root Cause**: The v0.52 parser enhancement for multiple assignment was overly aggressive in detecting comma-separated expressions and wrapping them in tuples, even when inside collection literals.

**Solution**: Implemented context-aware parsing using an `is_parsing_collection` flag that prevents tuple wrapping when parsing elements inside collections.

## Technical Details

### Parser Enhancement

Added context tracking to distinguish between:
1. Comma-separated values that should become tuples (in expressions)
2. Comma-separated values that are collection elements (in literals)

### Implementation
- New parser field: `is_parsing_collection: bool`
- Modified functions: `list()`, `dict_or_set_literal()`, `expr_list_or_tuple()`
- Context-aware logic in `assignment_or_lambda()`

### Multiple Variable Declarations Fix

**Issue**: After v0.52's multiple assignment implementation, multiple variable declarations (`var x, y = 10, 20`) only registered the first variable in the symbol table, causing "redeclaration" errors.

**Before v0.53** (Broken):
```frame
var x, y, z = 1, 2, 3  # Error: "redeclaration of 'y'" ❌
```

**After v0.53** (Fixed):
```frame
var x, y, z = 1, 2, 3  # Works correctly ✅
print("x=" + str(x) + " y=" + str(y) + " z=" + str(z))  # x=1 y=2 z=3

# Also works with unpacking
var tuple_val = (10, 20, 30)
var a, b, c = tuple_val  # Unpacks correctly ✅

# Mixed types
var name, age, score = "Alice", 25, 98.5  # All types work ✅
```

**Root Cause**: The parser's `handle_multiple_var_declaration` function only registered the first variable in the symbol table during the first pass.

**Solution**: Modified the parser to register ALL variables in the symbol table and use a special naming convention (`__multi_var__:name1,name2,name3`) to signal multiple declarations to the visitor.

## Testing

New test files:
- `framec_tests/python/src/test_v053_list_fix.frm` - Collection literal fixes
- `framec_tests/python/src/test_v053_multi_var.frm` - Multiple variable declarations

Test coverage includes:
- List literals with multiple elements
- Nested collections
- Dictionary literals
- Set literals
- Tuple literals
- Mixed collection types
- Multiple variable declarations
- Variable unpacking from tuples and lists
- Mixed single and multiple declarations

## Migration Guide

No migration required. Both fixes restore correct behavior that was broken in v0.52.

### Collection Literals
If you implemented workarounds for the collection literal bug:
```frame
# Workaround (no longer needed):
var lst = []
lst.append(1)
lst.append(2)

# Now works correctly:
var lst = [1, 2, 3]
```

### Multiple Variable Declarations
If you avoided multiple variable declarations:
```frame
# Workaround (no longer needed):
var x = 1
var y = 2
var z = 3

# Now works correctly:
var x, y, z = 1, 2, 3
```

## Compatibility

- **Backward Compatible**: ✅ Yes
- **Breaking Changes**: None
- **Minimum Frame Version**: v0.53

## Performance

No performance impact. The fix adds a simple boolean check during parsing with no runtime overhead.

## Contributors

- Parser context tracking implementation
- Comprehensive test coverage
- Documentation updates

## What's Next

Future enhancements under consideration:
- Extended unpacking patterns with wildcards (`var x, *rest, y = values`)
- Tuple unpacking in for loops (`for x, y in pairs`)
- Pattern matching in variable declarations

## References

- [v0.52 Release Notes](release_notes_v0.52.md) - Multiple assignment feature
- [v0.53 Achievements](v0.53_achievements.md) - Detailed technical documentation
- [Grammar Documentation](framelang_design/grammar.md) - Updated syntax rules
# Frame v0.31 - Complete Legacy Syntax Removal

**Release Date**: 2025-09-01  
**Branch**: v0.31  
**Status**: ✅ 100% Test Success (166/166 tests passing)

## Executive Summary

Frame v0.31 completes the modernization of the Frame language by **completely removing** all v0.11 legacy syntax. This is a breaking change that requires migration of any existing code using old syntax patterns. The transpiler now exclusively uses the modern v0.20+ syntax, making the language cleaner, more consistent, and easier to learn.

## Breaking Changes

### Completely Removed Syntax

All of the following v0.11 syntax elements have been removed and will cause **compilation errors**:

#### 1. Return Operators
- **REMOVED**: `^` (simple return)
- **REMOVED**: `^(value)` (return with value)
- **REMOVED**: `^=` (return assignment)
- **Use Instead**: 
  - `return` (simple return)
  - `return value` (return with value)
  - `return = value` (interface return assignment)

#### 2. Ternary Test Operators
- **REMOVED**: `?` (boolean test true)
- **REMOVED**: `?!` (boolean test false)
- **REMOVED**: `?~` (string match test)
- **REMOVED**: `?#` (number match test)
- **REMOVED**: `?:` (enum match test)
- **Use Instead**: Standard if/elif/else statements

#### 3. Test Terminators
- **REMOVED**: `:|` (test terminator)
- **REMOVED**: `::` (alternative test terminator)
- **Use Instead**: Not needed with if/elif/else

#### 4. Pattern Matching Operators
- **REMOVED**: `~/` (string pattern matching)
- **REMOVED**: `#/` (number pattern matching)
- **REMOVED**: `:/` (enum pattern matching)
- **Use Instead**: if/elif/else with explicit comparisons

#### 5. Old System Declaration
- **REMOVED**: `#SystemName ... ##`
- **Use Instead**: `system SystemName { ... }`

#### 6. Old Block Markers
- **REMOVED**: `-interface-`, `-machine-`, `-actions-`, `-domain-`, `-operations-`
- **Use Instead**: `interface:`, `machine:`, `actions:`, `domain:`, `operations:`

#### 7. Old Parameter Syntax
- **REMOVED**: `[param1, param2]` (bracket parameters)
- **Use Instead**: `(param1, param2)` (parenthesis parameters)

#### 8. Old Event Handler Syntax
- **REMOVED**: `|eventName|` (pipe event handlers)
- **REMOVED**: `|>|` and `|<|` (old enter/exit events)
- **Use Instead**: 
  - `eventName()` (standard event handlers)
  - `$>()` and `<$()` (enter/exit events)

#### 9. Old Attributes
- **REMOVED**: `#[static]` (Rust-style attributes)
- **Use Instead**: `@staticmethod` (Python-style attributes)

## Migration Guide

### Example Migrations

#### Old Return Syntax → Modern Return
```frame
// OLD (NO LONGER WORKS)
eventHandler() {
    ^(42)  // COMPILATION ERROR
}

// NEW (REQUIRED)
eventHandler() {
    return 42
}
```

#### Old Ternary → Modern If/Else
```frame
// OLD (NO LONGER WORKS)
x ? print("true") : print("false") :|

// NEW (REQUIRED)
if x {
    print("true")
} else {
    print("false")
}
```

#### Old Pattern Matching → Modern Comparisons
```frame
// OLD (NO LONGER WORKS)
s ?~ /foo|bar/ print("matched") :> : print("no match") :|

// NEW (REQUIRED)
if s == "foo" || s == "bar" {
    print("matched")
} else {
    print("no match")
}
```

#### Old System Declaration → Modern System
```frame
// OLD (NO LONGER WORKS)
#TrafficLight
    -machine-
    $Red
        |timer| -> $Green ^
##

// NEW (REQUIRED)
system TrafficLight {
    machine:
        $Red {
            timer() {
                -> $Green
                return
            }
        }
}
```

## Implementation Details

### Scanner Changes (scanner.rs)
- Removed TokenType enum variants:
  - `Caret`, `ReturnAssign`, `ColonBar`
  - `StringMatchStart`, `NumberMatchStart`, `EnumMatchStart`
  - `ThreeTicks` (unused)
- Added error generation for deprecated characters:
  - `^` → "Old return syntax has been removed"
  - `?` → "Ternary operators have been removed"
  - `#` → "Old system declaration syntax has been removed" (except for attributes)

### Parser Changes (parser.rs)
- Removed/commented out functions:
  - `bool_test()` - Returns error immediately
  - `string_match_test()` - Commented out entirely
  - `number_match_test()` - Commented out entirely
  - `enum_match_test()` - Commented out entirely
- Removed references to deleted tokens in sync token lists
- Simplified branch_terminator() to return None

### AST Changes (ast.rs)
- Commented out TestStatementNode struct (kept for reference)
- TestType enum still exists but is unused
- StatementType no longer includes TestStmt variant

### Visitor Changes
- All 13 visitor implementations updated
- Removed TestStmt handling from all visitors
- No functionality lost - all features available through modern syntax

## Error Messages

The compiler now provides helpful error messages when encountering removed syntax:

```
Line 10: Error: Unexpected character '^'. Old return syntax has been removed. Use 'return' or 'return value' instead.

Line 15: Error: Unexpected character '?'. Ternary operators have been removed. Use if/elif/else statements instead.

Line 20: Error: Unexpected character '#'. Old system declaration syntax has been removed. Use 'system Name { }' instead.
```

## Test Results

- **Total Tests**: 166
- **Passing**: 166
- **Success Rate**: 100%
- **Validation**: All tests executed and validated, not just transpiled

## Benefits

1. **Simpler Language**: One clear way to express each concept
2. **Better Readability**: Modern syntax is more explicit and familiar
3. **Reduced Complexity**: Fewer syntax variants to learn and maintain
4. **Cleaner Codebase**: Removed ~400 lines of deprecated code handling
5. **Future-Ready**: Clear foundation for future language enhancements

## Compatibility Notes

- **No Backward Compatibility**: Old syntax will not compile
- **Migration Required**: All existing Frame code must be updated
- **No Compatibility Mode**: No flags or options to enable old syntax
- **Clear Error Messages**: Compiler guides migration with specific error messages

## Next Steps

1. **Migrate Existing Code**: Update any Frame code using old syntax
2. **Update Documentation**: Ensure all examples use modern syntax
3. **Remove Test Code**: Clean up commented test-related code in future release
4. **Focus on Features**: With syntax stabilized, focus on new capabilities

## Technical Notes

- Scanner generates errors immediately for removed tokens
- Parser functions for ternary tests return errors or are commented out
- All visitor implementations handle only modern syntax
- Test suite validates complete functionality with modern syntax only

---

*Frame v0.31 represents the completion of the syntax modernization effort started in v0.20. The language now has a clean, consistent, and modern syntax that serves as a solid foundation for future development.*
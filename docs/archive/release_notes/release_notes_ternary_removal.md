# Frame Transpiler - Ternary Syntax Removal

## Release Date: 2025-01-01
## Branch: v0.30

## Breaking Change: Complete Removal of Ternary Operators

### Summary
All deprecated ternary operators and test terminators have been **completely removed** from the Frame language. This is a breaking change that requires migration of any existing code using these operators.

### Removed Syntax

#### Ternary Operators (REMOVED)
- `?` - Boolean test true
- `?!` - Boolean test false  
- `?~` - String match test
- `?#` - Number match test
- `?:` - Enum match test

#### Test Terminators (REMOVED)
- `:|` - Test terminator
- `::` - Alternative test terminator

### Migration Guide

All conditional logic must now use if/elif/else statements:

#### Before (No Longer Supported):
```frame
// Boolean test
x ? print("true") : print("false") :|

// String match
s ?~ /foo|bar/ print("matched") :> : print("no match") :|

// Number match  
n ?# /1|2|3/ print("matched") :> : print("no match") :|

// Enum match
e ?: (Color) /Red|Blue/ print("matched") :> : print("no match") :|
```

#### After (Required Syntax):
```frame
// Boolean test
if x {
    print("true")
} else {
    print("false")
}

// String match
if s == "foo" || s == "bar" {
    print("matched")
} else {
    print("no match")
}

// Number match
if n == 1 || n == 2 || n == 3 {
    print("matched")
} else {
    print("no match")
}

// Enum match
if e == Color.Red || e == Color.Blue {
    print("matched")
} else {
    print("no match")
}
```

### Impact

#### Compilation Behavior
- Any use of ternary operators (`?`, `?!`, `?~`, `?#`, `?:`) will cause **compilation failure**
- Clear error messages direct users to use if/elif/else statements
- Example error: "Unexpected character '?'. Ternary operators have been removed. Use if/elif/else statements instead."

#### Code Changes
- **Scanner**: Removed all ternary token types
- **Parser**: Removed all ternary parsing functions
- **AST**: Removed TestStmt from StatementType enum
- **Visitors**: Removed TestStmt handling from all language targets

### Test Results
- **100% test success rate** (166/166 tests passing)
- All tests validated with both transpilation and execution
- No functionality lost - all conditional logic expressible with if/elif/else

### Benefits
1. **Simpler Language**: One clear way to express conditionals
2. **Better Readability**: if/elif/else is more explicit and readable
3. **Reduced Complexity**: Fewer syntax variants to learn and maintain
4. **Modern Syntax**: Aligns with contemporary programming languages

### Files Modified
- `framec/src/frame_c/scanner.rs` - Removed ternary tokens
- `framec/src/frame_c/parser.rs` - Removed/commented ternary parsing
- `framec/src/frame_c/ast.rs` - Removed TestStmt variant
- `framec/src/frame_c/visitors/*.rs` - Removed TestStmt handling (13 files)
- `CLAUDE.md` - Updated documentation

### Note for Existing Projects
Projects using ternary operators will need to be migrated before they can be compiled with this version. The compiler provides clear error messages to identify all locations requiring updates.
# Changelog v0.86.13

**Release Date**: 2025-01-22  
**Type**: Bug-fix release  
**Focus**: TypeScript transpiler improvements

## Overview

This release delivers significant improvements to the TypeScript transpiler, achieving 75.8% execution success rate in the data_types category and resolving multiple critical syntax and runtime issues.

## ✅ Major Fixes

### Dictionary Complex Key Support
- **Issue**: JavaScript syntax errors with complex dictionary keys like `{[1, 2]: "value"}`
- **Fix**: Implemented computed property syntax with automatic string conversion
- **Impact**: Resolves dictionary literal compilation failures

### Set Equality Comparison  
- **Issue**: Set comparisons like `set1 == set2` always failed due to reference comparison
- **Fix**: Implemented intelligent deep equality comparison for Sets, arrays, and objects
- **Impact**: Set comprehension tests now pass completely
- **Details**: Uses selective detection to apply deep comparison only when needed, preserving simple `===` for primitives

### Call Chain Literal Processing
- **Issue**: String literals in method calls like `r"FRAME".toLowerCase()` failed to compile
- **Fix**: Enhanced CallChainLiteralExprT handler with comprehensive literal processing
- **Impact**: Raw string method calls now work correctly

### Lambda Function Compilation
- **Issue**: Dictionary/array lambda calls generated invalid `.@indexed_call()` syntax  
- **Fix**: Added special handling for @indexed_call synthetic nodes
- **Impact**: Lambda expressions in collections work correctly

### Type System Functions
- **Issue**: Missing `type()` function support in TypeScript
- **Fix**: Added type() function with constructor name detection
- **Impact**: Python-style type checking now available

### Range Function Improvements
- **Issue**: `range(2, 8)` generated invalid syntax
- **Fix**: Enhanced range() transpilation with proper 3-pattern handling
- **Impact**: All range() patterns now compile correctly

## 📊 Performance Metrics

- **Transpilation Success**: 100% (66/66 tests)
- **Execution Success**: 75.8% (50/66 tests) 
- **Improvement**: ~6 percentage point increase from v0.86.12
- **Target Achievement**: Exceeded 72.7% target, approaching 85% goal

## 🏗️ Technical Details

### Smart Equality Detection
The TypeScript visitor now uses intelligent detection for when to apply deep equality:
- **Sets/Arrays/Objects**: Deep comparison with content checking
- **Primitives/Expressions**: Simple `===` comparison for performance
- **Modulo operations**: Preserved simple comparison for `x % 2 == 0` patterns

### Dictionary Key Processing
Complex dictionary keys are automatically converted:
```typescript
// Frame: {[1, 2]: "value"}
// Generated: {[JSON.stringify([1, 2])]: "value"}
```

### Enhanced AST Processing
Improved handling of synthetic parser nodes and call chain expressions for better code generation.

## 🎯 Compatibility

This release maintains full backward compatibility while significantly improving TypeScript output quality and execution reliability.

## 🔄 Next Steps

With data_types category achieving 75.8% success rate, focus areas for future releases:
- Remaining dictionary method implementations
- Collection constructor edge cases  
- Advanced string operations
- Target: 85%+ overall execution success rate
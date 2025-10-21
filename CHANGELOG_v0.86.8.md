# Frame Transpiler v0.86.8 Release Notes

## Release Date: 2025-01-21

## Overview
Bug fix release improving TypeScript visitor code generation with multiple critical fixes for proper TypeScript syntax generation.

## 🐛 Bug Fixes

### TypeScript Visitor Improvements

1. **Fixed Multiple Return Value Handling**
   - Corrected invalid syntax generation for functions returning multiple values
   - Now properly generates array destructuring: `let [a, b]: any[] = functionCall()`
   - Previously generated invalid: `let __multi_var__:a,b: any = functionCall()`

2. **Enhanced String Multiplication Support**
   - Extended string repetition to handle variables and expressions, not just literals
   - Generates `(expression).repeat(count)` for any string expression multiplied by a number
   - Works with both `string * number` and `number * string` patterns

3. **Added Missing Statement Type Handlers**
   - **RaiseStmt**: Properly generates `throw new Error(...)` statements
   - **DelStmt**: Outputs explanatory comment since JavaScript lacks direct equivalent
   - **AssertStmt**: Converts to runtime checks: `if (!(condition)) { throw new Error('Assertion failed') }`

4. **Improved Python Keyword Handling**
   - Fixed `pass` statement handling - now properly converts to `// pass` comment
   - Applied fix in all AST contexts (VariableExprT, UndeclaredIdentifierNodeT, CallChainNodeT)
   - Proper conversion of `True`/`False` to `true`/`false`

5. **Fixed Destructuring Syntax**
   - Corrected TypeScript array destructuring type annotations
   - Now generates valid `let [a, b]: any[] = ...` instead of invalid `let [a, b: any]: any[] = ...`

## 📊 Impact
- Significantly reduces TypeScript compilation errors
- Improves compatibility with comprehensive test suites
- Brings TypeScript visitor closer to feature parity with Python visitor

## 🔧 Technical Details
- All fixes implemented in `framec/src/frame_c/visitors/typescript_visitor.rs`
- No breaking changes to existing functionality
- Maintains backward compatibility with all previous versions

## 📈 Test Results
- Multiple comprehensive test files now compile successfully
- Reduced syntax errors in generated TypeScript code
- Better handling of Frame language constructs in TypeScript output

## Known Issues
- Bug #57: Throw statements with variable references are split across lines (documented in `docs/bugs/open/bug_057_typescript_throw_splitting.md`)

## Next Steps
- Fix throw statement line splitting issue (Bug #57)
- Continue improving TypeScript visitor for 100% test success rate
- Add more comprehensive type inference for better TypeScript generation
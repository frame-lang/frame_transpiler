# Bug #57: TypeScript Throw Statement Line Splitting

## Status: OPEN
## Priority: Medium
## Component: TypeScript Visitor
## Version: v0.86.8

## Description
When Frame code contains a pattern like:
```frame
var error = "Error message"
throw error
```

The TypeScript visitor generates:
```typescript
let error = "Error message";
throw;
error;
```

This creates invalid TypeScript syntax as `throw;` expects an expression on the same line.

## Root Cause
The parser treats the variable declaration and throw statement as two separate statements. The TypeScript visitor correctly generates each statement individually but doesn't recognize the pattern where a throw statement immediately follows an error variable declaration.

## Expected Behavior
Should generate:
```typescript
let error = "Error message";
throw error;
```

Or combine into:
```typescript
throw new Error("Error message");
```

## Workaround
Write throw statements with inline expressions in Frame:
```frame
throw "Error message"
```

## Proposed Solution
1. In the TypeScript visitor's RaiseStmt handler, check if the exception expression is a simple variable reference
2. If so, generate `throw variableName;` instead of just `throw;`
3. Alternatively, implement a pattern detector that combines adjacent variable declaration + throw statements

## Test Cases
- `framec_tests/common/tests/systems/test_debug_simple.frm` - Lines 490-493
- `framec_tests/common/tests/systems/test_exceptions_basic.frm`

## Related Issues
- Part of broader TypeScript visitor improvements in v0.86.8
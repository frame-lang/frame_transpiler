# Bug #049: TypeScript Transpilation Rate Lower Than Python

## Metadata
```yaml
bug_number: 049
title: "TypeScript Transpilation Rate Lower Than Python"
status: Open
priority: Medium
category: CodeGen
discovered_version: v0.82.2
fixed_version: 
reporter: Test Suite Analysis
assignee: Claude
created_date: 2025-10-13
resolved_date: 
```

## Description
TypeScript transpilation success rate is significantly lower than Python. Many tests that work in Python fail to generate valid TypeScript code due to incomplete visitor implementation.

## Reproduction Steps
1. Run test suite with both Python and TypeScript targets
2. Compare success rates
3. Observe TypeScript has lower success rate

## Test Case
Multiple test files fail TypeScript generation while succeeding in Python:
- Files with f-strings
- Files with complex expressions
- Files with state parameters
- Files with domain variables

## Expected Behavior
TypeScript should have similar transpilation success rate to Python (at least 90%+ for common features).

## Actual Behavior
TypeScript transpilation fails on many common Frame patterns that work in Python.

## Impact
- **Severity**: Medium - Limits TypeScript adoption
- **Scope**: ~50% of tests failing for TypeScript
- **Workaround**: Use Python target instead

## Technical Analysis
The TypeScript visitor has incomplete implementations for:
1. F-string to template literal conversion
2. Domain variable handling
3. State parameter access
4. Complex expression patterns
5. Collection operations

### Root Cause
TypeScript visitor was developed later and hasn't kept pace with Python visitor features.

### Affected Files
- `framec/src/frame_c/visitors/typescript_visitor.rs`

## Proposed Solution
Systematically fix TypeScript visitor issues identified in Bug #47 and enhance test coverage.

## Test Coverage
- [ ] Fix f-string conversion
- [ ] Fix domain variables
- [ ] Fix state parameters
- [ ] Add TypeScript-specific tests
- [ ] Achieve 90%+ transpilation rate

## Related Issues
- Bug #047: TypeScript Complex Expression Support (partial fix)

## Work Log
- 2025-10-13: Bug documented

---
*Bug tracking policy version: 1.0*
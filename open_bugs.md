# Open Bugs

This file tracks known bugs and issues in the Frame transpiler.

---

## Bug #48: Unreachable Return After Transition Statements

**Status**: Open  
**Priority**: High  
**Affected Version**: v0.81.6  
**Components**: Parser, Semantic Analysis  
**Target Languages**: All (Python, TypeScript confirmed)

### Description
The transpiler incorrectly rejects valid Frame code that has `return` statements after state transitions (`->` operator). These returns are actually reachable in certain contexts but the transpiler's control flow analysis treats transitions as terminal statements.

### Affected Tests
Six core tests are failing with "Unreachable code: 'return' statement after trans" errors:

1. **test_event_handler.frm** (line 38, 45)
   - Pattern: `-> $S2(a+b)` followed by `return`
   - Use case: Event handlers with transitions

2. **test_state_context.frm** (line 37)
   - Pattern: Transition followed by return in state context
   - Use case: State context management

3. **test_state_context_stack.frm** (line 48)
   - Pattern: State stack operations with transitions
   - Use case: Hierarchical state machines

4. **test_state_params.frm** (line 15)
   - Pattern: `-> $Split(1)` followed by `return`
   - Use case: Parameterized state transitions

5. **test_state_stack.frm** (line 39)
   - Pattern: Stack manipulation with transitions
   - Use case: State history mechanisms

6. **test_transition_params.frm** (line 13)
   - Pattern: Parameterized transitions with returns
   - Use case: Data passing between states

### Example Code
```frame
# Current code that fails
$S1 {
    PassAdd(a:int, b:int) {
        -> $S2(a+b)
        return        # Error: Unreachable code
    }
}
```

### Root Cause Analysis
The transpiler's control flow analyzer treats state transitions as unconditional jumps that never return, similar to `throw` statements. However, in Frame's semantics:
1. Transitions schedule a state change but may not immediately transfer control
2. The return statement can be used to exit the current handler cleanly
3. Some runtime implementations may need the return for proper stack unwinding

### Proposed Solutions

#### Solution 1: Allow Return After Transition (Recommended)
- Treat transitions as non-terminal statements in control flow analysis
- Allow optional return statements after transitions
- Generate appropriate code for each target language

#### Solution 2: Make Return Implicit
- Automatically insert returns after transitions during code generation
- Remove explicit returns from test files
- Document this behavior clearly

#### Solution 3: Context-Aware Analysis
- Analyze whether the return is actually needed based on:
  - Handler return type
  - Target language requirements
  - Runtime implementation details

### Workaround
Remove the `return` statements after transitions in the affected test files. However, this may not match the intended Frame semantics.

### Impact
- **Test Suite**: 6 core tests failing (10% of core tests)
- **Languages**: Affects all target languages
- **Users**: May block valid Frame patterns

### References
- Test files in: `framec_tests/common/tests/core/`
- Related issues: Possibly related to state machine semantic rules

---

## Bug #49: TypeScript Transpilation Rate Lower Than Python

**Status**: Open  
**Priority**: Medium  
**Affected Version**: v0.82.0  
**Components**: TypeScript Visitor  
**Target Languages**: TypeScript

### Description
TypeScript has a lower transpilation success rate (80.6%) compared to Python (90%) on the same core tests, indicating missing or incorrect implementations in the TypeScript visitor.

### Test Results
- Python: 54/60 core tests passing (90.0%)
- TypeScript: 25/31 core tests passing (80.6%)

### Analysis Needed
1. Identify which specific tests fail for TypeScript but pass for Python
2. Determine if these are visitor implementation issues or language-specific constraints
3. Update TypeScript visitor to match Python visitor capabilities

---

## Bug #50: Language-Specific Tests Running for All Languages

**Status**: Fixed  
**Priority**: Low  
**Affected Version**: Current  
**Components**: Test Runner  
**Target Languages**: All  
**Fixed In**: 2025-10-13  

### Description
The test runner was incorrectly including Python-specific tests (29 tests) when running "core" category tests. Language-specific tests should only run for their respective languages.

### Expected Behavior
- Common tests run for all specified languages
- Language-specific tests only run for their specific language
- Categories should not include language-specific tests unless explicitly requested

### Current Behavior
When running `--categories core`, the test runner also includes all Python-specific tests from `language_specific/python/`.

### Fix Applied
Updated the `discover_tests()` method in `frame_test_runner.py` to:
1. Only include language-specific tests when "all" categories is specified
2. Allow explicit selection of language-specific tests by category name
3. Exclude language-specific tests from common category runs

### Verification
```bash
# Before fix: showed 60 tests (31 core + 29 Python-specific)
# After fix: shows 31 tests (core only)
python3 framec_tests/runner/frame_test_runner.py --languages python --categories core

# Language-specific tests still work when requested
python3 framec_tests/runner/frame_test_runner.py --categories language_specific_python
```

---

## Notes for Contributors

### How to Reproduce Bugs
```bash
# Bug #48 - Unreachable return errors
./run_tests.sh --categories core --transpile-only

# Bug #49 - TypeScript transpilation issues  
./run_tests.sh --languages typescript --categories core

# Bug #50 - Language-specific test inclusion
python3 framec_tests/runner/frame_test_runner.py --languages typescript --categories core
# Notice Python-specific tests are included
```

### Testing After Fixes
```bash
# Run full test suite
./run_tests.sh --all

# Run specific category
./run_tests.sh --categories core --verbose

# Generate report
./run_tests.sh --output test_results.json
```

---

*Last Updated: 2025-10-13*
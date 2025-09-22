# Source Map Bug Report - v0.70

## Bug Status
**Priority**: High  
**Version Affected**: v0.69, v0.70  
**Accuracy**: ~50% (needs to be 100%)

## Information Needed to Fix

### 1. Specific Symptoms
Please provide examples of what's happening:
- [ ] Are breakpoints hitting the wrong lines? Which lines?
- [ ] Are error messages showing incorrect line numbers?
- [ ] Is step-through debugging jumping to unexpected locations?

### 2. Test Case
Please provide a minimal Frame file that demonstrates the issue:
```frame
# Your test case here
```

### 3. Expected vs Actual Behavior
For the test case above:
- **Frame Line X**: `[your code]`
  - **Expected**: Should map to Python line Y
  - **Actual**: Maps to Python line Z

### 4. Debugging Output
Please run with debug flag and provide output:
```bash
FRAME_TRANSPILER_DEBUG=1 ./target/release/framec -l python_3 --debug-output your_test.frm > debug.json
```

### 5. Most Critical Issues
Which of these is most problematic?
- [ ] Function/method mappings
- [ ] Statement mappings inside event handlers
- [ ] Transition statement mappings
- [ ] Action/operation call mappings
- [ ] Other: _______________

### 6. IDE/Debugger Used
- [ ] VSCode with Python extension
- [ ] PyCharm
- [ ] Command line debugger (pdb)
- [ ] Other: _______________

## Known Issues from v0.69

### Still Broken (50% of mappings)
1. **Statements inside event handlers**: Off by ±1 line
2. **Action block mappings**: Off by one line  
3. **Transition statement mappings**: Map to incorrect locations
4. **Double newline between handlers**: Causes accumulating offset errors

### Already Fixed (50% of mappings)
1. ✅ Function definitions correctly mapped
2. ✅ Simple statements in functions correctly mapped
3. ✅ Event handler definitions correctly mapped

## Root Causes Identified
1. **Inconsistent newline generation**: Different code structures use different newline patterns
2. **Multiple mapping sources**: Some AST nodes get mapped multiple times
3. **Line counter desynchronization**: `current_line` tracker gets out of sync with actual output

## Proposed Fix Approach

### Phase 1: Diagnose Exact Issue
Need the information requested above to pinpoint the exact problem.

### Phase 2: Fix Approach Options
1. **Option A**: Standardize all newline generation to use `newline_and_map()`
2. **Option B**: Implement line tracking verification system
3. **Option C**: Rewrite mapping system with single-responsibility principle

### Phase 3: Validation
1. Create comprehensive source map tests
2. Validate all statement types map correctly
3. Test with real debugging scenarios

## Notes for Developer
- The v0.70 changes (adding blank lines) may have made the problem worse by adding more newlines
- The `newline_and_map()` method from v0.69 is a good pattern but not used everywhere
- Consider whether we need to track "pending newlines" vs "actual newlines"

---

**Please fill in the requested information above so we can fix this bug properly.**
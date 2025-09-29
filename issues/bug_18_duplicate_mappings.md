# Bug #18: Domain Variable Duplicate Source Mappings

**Bug ID:** #18  
**Date Reported:** 2024-12-29  
**Last Updated:** 2024-12-30  
**Severity:** Low (reduced from High)  
**Status:** Partially Fixed - 71% improvement achieved  
**Current Version:** v0.78.18  

## Summary
Domain variable declarations in Frame systems generate duplicate source map entries, causing the VS Code debugger to incorrectly jump to domain variable lines when stepping through system initialization code.

## Current Status (v0.78.18)
- **Original (v0.78.14):** 7 duplicate mappings for line 37
- **Current (v0.78.15-18):** 2 duplicate mappings for line 37
- **Improvement:** 71% reduction in duplicates
- **Remaining Issue:** Frame line 37 still maps to Python lines 40 and 42

## Test Case
File: `test_none_keyword.frm`
```frame
system NoneChecker {
    // ... interface and machine sections ...
    
    domain:
        var data = None  // Line 37 - This line has duplicate mappings
}
```

## How to Reproduce
```bash
# Generate debug output and count mappings for line 37
framec -l python_3 --debug-output test_none_keyword.frm | grep '"frameLine": 37' | wc -l
# Result: 2 (should be 0 or 1)

# See the duplicate mappings
framec -l python_3 --debug-output test_none_keyword.frm | grep '"frameLine": 37' -A1 -B1
# Shows mappings to Python lines 40 and 42
```

## Detection Method Used by VS Code Extension

### 1. Automated Detection Script
```python
#!/usr/bin/env python3
import json
import subprocess
from collections import defaultdict

def detect_duplicate_mappings(frm_file):
    """Detect Frame lines with multiple Python line mappings"""
    result = subprocess.run(
        ['framec', '-l', 'python_3', '--debug-output', frm_file],
        capture_output=True, text=True
    )
    data = json.loads(result.stdout)
    
    # Group by Frame line
    frame_to_python = defaultdict(list)
    for mapping in data['sourceMap']['mappings']:
        frame_to_python[mapping['frameLine']].append(mapping['pythonLine'])
    
    # Find duplicates
    duplicates = {}
    for frame_line, python_lines in frame_to_python.items():
        if len(python_lines) > 1:
            duplicates[frame_line] = python_lines
            
    return duplicates

# Test for Bug #18
duplicates = detect_duplicate_mappings('test_none_keyword.frm')
for frame_line, python_lines in duplicates.items():
    print(f"Frame line {frame_line} maps to {len(python_lines)} Python lines: {python_lines}")
```

### 2. Why This Matters for Debugging
When the VS Code debugger steps through Python code:
1. Python debugger reports current line (e.g., line 42)
2. Extension looks up line 42 in source map
3. Finds it maps to Frame line 37 (domain variable)
4. IDE highlights line 37 instead of the actual executing code
5. User sees debugger "jump backwards" to domain section
6. Creates confusing debugging experience

## Expected Behavior
Domain variable declarations should either:
1. **Option A:** Have exactly ONE mapping to the Python line where `self.data = None` is executed
2. **Option B:** Have NO mappings (preferred, since domain declarations are not executable statements)

## Impact on Users
- Debugger appears to jump to wrong lines during system initialization
- Step-over operations show incorrect line highlights
- Makes debugging Frame systems confusing
- **Severity reduced** from High to Low due to 71% improvement

## Root Cause Analysis
The transpiler appears to call `add_source_mapping()` multiple times when generating the `__init__` method for systems with domain variables. Each call creates a mapping from the domain variable line to different parts of the generated Python code.

### Likely Code Location
In `python_visitor_v2.rs`, look for:
- Domain variable processing in system generation
- Multiple calls to `add_source_mapping()` with the same Frame line number
- `__init__` method generation for systems

## Suggested Fix
1. Track which Frame lines have already been mapped during `__init__` generation
2. Only create one mapping per Frame line (or none for declarative statements)
3. Consider treating domain variable declarations as non-executable (no mappings)

## Test Verification
After fix, verify:
```bash
# Should return 0 or 1, not 2
framec -l python_3 --debug-output test_none_keyword.frm | grep '"frameLine": 37' | wc -l
```

## Additional Context
- All 376 Frame tests pass despite this issue
- This is a debugging QoL issue, not a correctness problem
- The 71% improvement in v0.78.15 shows the fix is on the right track
- Just need to eliminate the final 2 duplicate mappings

## Files Affected
- `framec/src/frame_c/visitors/python_visitor_v2.rs` - Domain variable processing
- `framec_tests/python/src/positive_tests/test_none_keyword.frm` - Test case

## Priority
Low - The issue is much improved and only affects debugging experience during system initialization. All tests pass and code executes correctly.
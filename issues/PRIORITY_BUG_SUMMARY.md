# Priority Bug Summary for Frame Transpiler Team

**Date:** 2024-12-30  
**From:** VS Code Frame Extension Team  
**Current Version:** v0.78.18  

## Bugs Requiring Transpiler Team Action

### 🟡 Bug #18: Domain Variable Duplicate Source Mappings
**Priority:** Low  
**File:** `bug_18_duplicate_mappings.md`  

**Quick Summary:**
- Domain variable line (37) maps to 2 Python lines (40, 42)
- Already improved 71% (was 7 duplicates, now 2)
- Causes minor debugger confusion during system initialization

**Action Needed:**
- Review domain variable processing in `python_visitor_v2.rs`
- Ensure `add_source_mapping()` called at most once per Frame line
- Consider making domain declarations non-mappable (they're not executable)

**Verification:**
```bash
# Should return 0 or 1, currently returns 2
framec -l python_3 --debug-output test_none_keyword.frm | grep '"frameLine": 37' | wc -l
```

---

## Bugs NOT Requiring Transpiler Action

### ⚠️ Bug #11: VS Code Debugger Line Offset
**Priority:** N/A for transpiler  
**File:** `bug_11_debugger_offset.md`  

**Quick Summary:**
- VS Code extension issue, NOT transpiler bug
- Extension injects ~700 lines of debug code, shifting line numbers
- Transpiler source maps are correct

**Action Needed:** None - VS Code extension team will fix

---

## Success Summary

### ✅ Recently Fixed (v0.78.15-18)
- Bug #16: Circular import detection - FULLY FIXED
- Bug #19: Parser error with functions after systems - FIXED
- Bug #20: Parser error with state parameters - FIXED
- 100% test pass rate achieved (376/376 tests passing)

### 📊 Current Quality Metrics
- **Test Success:** 100% (376/376 passing)
- **Source Map Quality:** 98% (only minor duplicates remain)
- **Parser Stability:** Excellent
- **Production Ready:** Yes

---

## Detection Tools Provided

We've included automated detection scripts in the bug reports to help verify fixes:

```python
# detect_duplicates.py - Finds all duplicate source mappings
import json
import subprocess
from collections import defaultdict

def detect_duplicate_mappings(frm_file):
    result = subprocess.run(
        ['framec', '-l', 'python_3', '--debug-output', frm_file],
        capture_output=True, text=True
    )
    data = json.loads(result.stdout)
    
    frame_to_python = defaultdict(list)
    for mapping in data['sourceMap']['mappings']:
        frame_to_python[mapping['frameLine']].append(mapping['pythonLine'])
    
    duplicates = {k: v for k, v in frame_to_python.items() if len(v) > 1}
    return duplicates

# Run on any .frm file to find issues
```

---

## Files in This Directory

1. **bug_18_duplicate_mappings.md** - Detailed report on remaining duplicate mappings
2. **bug_11_debugger_offset.md** - Documentation of VS Code extension issue (FYI only)
3. **v0.78.18_status_report.md** - Complete status report with all improvements
4. **PRIORITY_BUG_SUMMARY.md** - This file

---

## Next Steps

Only **Bug #18** needs transpiler team attention, and it's low priority since:
- It's 71% fixed already
- Only affects debugging experience
- All tests pass
- Code executes correctly

Thank you for the excellent improvements in v0.78.15-18! The 100% test pass rate is a major achievement.
# Frame Transpiler Open Bugs

**Last Updated:** 2024-12-30  
**Current Version:** v0.78.19  
**Active Bugs:** 1 (Bug #11 - VS Code extension issue only)  
**Resolved Bugs:** 26 (including #18 fully resolved in v0.78.19)

## VS Code Extension Testing Session Summary (2024-12-30)

### Versions Tested
- Started with v0.78.13, progressed through v0.78.15, v0.78.17, ending with v0.78.18
- VS Code extension rebuilt and tested with each version

### Key Findings
1. **Bug #18 Progress**: Domain variable duplicate mappings reduced from 7 to 2 (71% improvement)
   - v0.78.14: 7 duplicates
   - v0.78.15-18: 2 duplicates remain
   - Impact: Minor debugging inconvenience only

2. **Bug #11 Confirmed**: Debugger line offset is VS Code extension architecture issue
   - Not a transpiler bug
   - Extension injects ~700 lines of debug instrumentation
   - Source maps don't account for this offset
   - Fix required in VS Code extension, not transpiler

3. **Major Fixes Verified**:
   - Bug #19 & #20: Parser errors with functions after systems - CONFIRMED FIXED
   - Bug #16: Circular import detection - CONFIRMED FULLY FIXED
   - All 376 tests pass (100% success rate)

### VS Code Extension Improvements Made
- Implemented multi-state machine architecture for debugger
- Refactored to use stdin (eliminated temp files)  
- Fixed frame tracking for step operations
- Improved error handling and state management

### Detection Methods Documented
- Added automated scripts to detect duplicate mappings
- Documented how VS Code extension detects source map issues
- Provided verification methods for transpiler team

## Bug Summary - All Tests Now Passing!

### All Transpilation Tests Fixed
- **Bug #19**: test_python_logical_operators.frm - **RESOLVED in v0.78.15**
- **Bug #20**: test_state_parameters_simple.frm - **RESOLVED in v0.78.15**

**Overall Test Status**: 100% pass rate (376/376 tests passing)

## Recent Improvements

### v0.78.18
- ✅ Fixed Bug #16: Circular dependency error messages now completely clean
- ✅ No more duplicate modules in circular dependency errors
- ✅ Error messages show clean cycles: "A → B → A" instead of "A → B → A → A"
- ✅ All 376 tests passing (100% success rate maintained)

### v0.78.17
- ✅ Improved circular dependency path cleaning
- ✅ Removed redundant ./ prefixes from module paths
- ✅ All tests continue to pass

### v0.78.16  
- ⚠️ Partially Fixed Bug #18: Domain variable duplicate mappings reduced (but not to 0 as claimed)
- ✅ Fixed Bug #16: Circular import detection now shows actual module paths instead of "unknown"
- ✅ Improved circular dependency error messages with meaningful module names
- ✅ Test pass rate maintained at 100% (376/376 passing)

### v0.78.15
- ✅ Fixed Bug #19: test_python_logical_operators.frm - Parser bug with functions after systems resolved
- ✅ Fixed Bug #20: test_state_parameters_simple.frm - Same parser bug resolved
- ✅ Fixed duplicate source mappings for cleaner debugging
- ✅ Test pass rate improved from 99.5% to 100% (376/376 passing)
- ✅ All Frame test suite tests now pass!

### v0.78.14
- ✅ Fixed Bug #21: test_static_calls.frm - cross-system static calls now correct
- ✅ Fixed Bug #22: test_static_comprehensive_v062.frm - same static call fix
- ✅ Test pass rate improved from 98.9% to 99.5% (374/376 passing)

### v0.78.13
- ✅ Fixed Bug #17: Module-level system instantiation now properly detected
- ✅ Fixed Bug #14a: test_call_chain_debug.frm - operation-to-operation calls fixed
- ✅ Fixed Bug #14b: test_seat_booking_simple_working.frm - now passes
- ✅ Test pass rate improved from 97.6% to 98.9% (372/376 passing)

### v0.78.1
- ✅ Fixed Bug #10: Static method call generation with incorrect "self" prefix
- ✅ Improved call chain handling for static methods
- ✅ All known bugs resolved

### v0.77.0
- ✅ Bug #9 resolved - removed debug output from regular transpilation
- ✅ Added interface definition source mappings for better debugging experience
- ✅ All debug eprintln! statements commented out in PythonVisitorV2

### v0.76.0
- ✅ Complete source mapping for ALL statement types
- ✅ Bug #8 resolved - statements in event handlers now have mappings
- ✅ Added mappings to 20+ statement visitor methods
- ✅ Zero active bugs - all known issues resolved

### v0.75.0
- ✅ CodeBuilder architecture implemented for automatic line tracking
- ✅ PythonVisitorV2 now default with robust source mapping
- ✅ Event handler declarations correctly map to function definitions

### v0.74.0
- ✅ Comprehensive source map architecture documentation added
- ✅ Marker file linter implemented for validation of intermediate files




### Bug #11: Debugger highlights wrong line when stepping through code

**Date Reported:** 2025-01-27  
**Severity:** Medium  
**Status:** ACTIVE 🔴 (VS Code Extension Issue - Not Transpiler)

#### Problem Description
When stepping through Frame code in the VS Code debugger, the highlighted line is often one line behind the actual line about to be executed. The debugger stops at the correct location but visually highlights the previous line.

#### Test Case
File: `test_none_keyword.frm`
```frame
system NoneChecker {
    // ... interface and machine sections ...
    
    domain:
        var data = None  // Line 37
}
```

#### Observed Behavior
Frame line 37 (`var data = None`) is mapped to multiple Python lines within the `__init__` method:
- Python line 40, 42, 44, 45, 46, 52, 53

When stepping through the `__init__` method, the debugger incorrectly shows execution at Frame line 37 for all these Python lines.

**Specific debugging issue:**
- Set breakpoint at Frame line 44 (`print("Function returned None")`)
- Step over from line 44
- **Expected**: Should advance to line 47 (next executable line)
- **Actual**: Jumps backward to line 37 (domain variable declaration)
- Call stack shows three `__init__` entries all mapped to line 37

#### Expected Behavior
Domain variable initialization lines should either:
1. Map to the specific Python line where the variable is initialized (e.g., `self.data = None`)
2. Have no mapping at all (since they're not directly executable Frame code)

#### Root Cause Analysis
The transpiler appears to be generating a source mapping for the domain variable declaration line every time it generates code within the `__init__` method. This results in one Frame line (37) having 7+ mappings to different Python lines.

#### Impact
- Debugger jumps to wrong Frame lines during system initialization
- Confusing debugging experience when stepping through constructors
- Makes it appear that execution is stuck on the domain variable line

#### Verification
```bash
# Check mappings for line 37
framec -l python_3 --debug-output test_none_keyword.frm | grep '"frameLine": 37' -A1 -B1
```

Result shows 7 different Python lines all mapped to Frame line 37:
```json
{ "frameLine": 37, "pythonLine": 40, "type": "function_def" }
{ "frameLine": 37, "pythonLine": 42, "type": "function_def" }
{ "frameLine": 37, "pythonLine": 44, "type": "function_def" }
{ "frameLine": 37, "pythonLine": 45, "type": "function_def" }
{ "frameLine": 37, "pythonLine": 46, "type": "function_def" }
{ "frameLine": 37, "pythonLine": 52, "type": "function_def" }
{ "frameLine": 37, "pythonLine": 53, "type": "function_def" }
```

#### Likely Fix Location
In the transpiler's Python visitor, the domain variable processing likely calls `add_source_mapping()` multiple times within the `__init__` generation code. Should be called once (or not at all) for domain variable declarations.

#### Bug Detection Methodology (How VS Code Extension Detects This)

**1. Source Map Analysis During Debugging:**
```bash
# Command used to detect duplicate mappings:
framec -l python_3 --debug-output test_none_keyword.frm | grep '"frameLine": 37' | wc -l
# v0.78.14: Returns 7 (bug present)
# v0.78.15: Returns 2 (partially fixed)
```

**2. VS Code Extension Detection Process:**
- When user sets breakpoint or steps through code, extension receives Python line number from debugger
- Extension looks up Python line in source map to find corresponding Frame line
- Multiple Python lines mapping to same Frame line causes incorrect jumps
- Extension detects this when debugger reports being at Python lines 40, 42, 44, etc. but all map to Frame line 37

**3. Automated Detection Script for Transpiler Team:**
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

# Usage: Detects bugs like #18
duplicates = detect_duplicate_mappings('test_none_keyword.frm')
for frame_line, python_lines in duplicates.items():
    print(f"Frame line {frame_line} maps to {len(python_lines)} Python lines: {python_lines}")
```

**4. Why This Detection Matters:**
- Duplicate mappings break debugger step operations
- Users see execution jumping to wrong Frame lines

### Bug #15: Set Constructor Incorrect Transpilation

**Date Reported:** 2025-01-28  
**Date Resolved:** 2025-09-28 (v0.78.12)  
**Severity:** High  
**Status:** RESOLVED ✅

#### Problem Description
The transpiler generated invalid Python code for `set()` constructor with multiple arguments. It generated `set(1, 2, 3)` instead of `set([1, 2, 3])`.

#### Test Case
Frame code:
```frame
var s = set(1, 2, 3)  # Multiple args to set constructor
```

Generated Python (BEFORE):
```python
s = set(1, 2, 3)  # Invalid Python - TypeError
```

Generated Python (AFTER):
```python
s = set([1, 2, 3])  # Correct - single iterable argument
```

#### Root Cause Analysis
The issue was that `set()` constructor calls were being processed through the UndeclaredCallT handler in call chains, not the direct CallExprT handler where the original fix was located.

#### Solution Implemented (v0.78.12)
Added special case logic to the UndeclaredCallT handler in `visit_call_chain_expr_node_to_string()`:
```rust
// Handle set() and frozenset() with multiple args
if (func_name == "set" || func_name == "frozenset") && call_node.call_expr_list.exprs_t.len() > 1 {
    output.push_str("([");
    for (i, expr) in call_node.call_expr_list.exprs_t.iter().enumerate() {
        if i > 0 {
            output.push_str(", ");
        }
        expr.accept_to_string(self, output);
    }
    output.push_str("])");
} else {
    // Normal argument processing
    // ...
}
```

#### Test Results
- **Before Fix**: Test `test_all_8_collection_patterns.frm` failed with `TypeError`
- **After Fix**: Test passes successfully, outputs "=== All 8 patterns working! ==="
- **Test Status**: Increased from 365/369 to 366/369 passing tests

#### Files Modified
- `framec/src/frame_c/visitors/python_visitor_v2.rs` - Added special case in UndeclaredCallT handler



### Bug #11: Debugger highlights wrong line when stepping through code

**Date Reported:** 2025-01-27  
**Severity:** Medium  
**Status:** ACTIVE 🔴

#### Problem Description
When stepping through Frame code in the VS Code debugger, the highlighted line is often one line behind the actual line about to be executed. The debugger stops at the correct location but visually highlights the previous line.

#### Test Case
Frame code (test_operations.frm):
```frame
fn main() {
    print("=== Testing Frame Operations ===")  # Line 6
    
    # Test instance operations
    var service = TestService()                 # Line 9
}
```

#### Observed Behavior
- Debugger stops before executing line 9 (`var service = TestService()`)
- VS Code highlights line 6 (`print("=== Testing Frame Operations ===")`) instead
- Execution is at the correct place (Python line 22), but visual indicator is wrong

#### Expected Behavior
- When stopped before executing line 9, line 9 should be highlighted
- The highlight should show the line ABOUT to be executed, not the last executed line

#### Technical Details
Source mappings are correct:
- Frame line 6 → Python line 21 (print statement)
- Frame line 9 → Python line 22 (TestService instantiation)

When Python debugger reports it's at line 22 (about to execute), the Frame debugger correctly maps this to Frame line 9, but the VS Code UI highlights line 6 instead.

#### Impact
- Confusing debugging experience
- User can't visually see which line is about to execute
- Makes step-by-step debugging difficult to follow

#### Root Cause Analysis (CONFIRMED)
The issue is caused by debug instrumentation that shifts Python line numbers:

1. **Original Generated Code**: 
   - Frame line 6 → Python line 21 (print statement)
   - Frame line 9 → Python line 22 (TestService instantiation)

2. **Instrumented Debug Code**:
   - Frame line 6 → Python line 728 (shifted by ~700 lines!)
   - Frame line 9 → Python line 729

3. **The Problem**: 
   - VS Code extension injects ~700 lines of debug instrumentation (FrameDebugger class, etc.)
   - This shifts all actual code down by hundreds of lines
   - Source maps are generated for the ORIGINAL code (line 21-22)
   - But debugger reports INSTRUMENTED line numbers (line 728-729)
   - The mapping fails because source maps don't account for instrumentation

#### Why This Happens
The VS Code Frame extension appears to:
1. Take the original generated Python code
2. Inject debugging instrumentation at the top (FrameDebugger class, trace hooks, etc.)
3. Run this instrumented version
4. Report line numbers from the instrumented version
5. Try to map these back using source maps built for the non-instrumented version

#### Solution Required
The VS Code extension needs to either:
- **Option 1**: Track the instrumentation offset and subtract it before mapping
- **Option 2**: Generate adjusted source maps that account for instrumentation
- **Option 3**: Use a different injection method that doesn't shift line numbers

#### Workaround
None available. This is a fundamental architectural issue in how debug instrumentation is handled.


---

## Debugging Tools & Process Notes

### How Source Map Issues Are Discovered

This section documents the systematic process and tools used to identify and analyze source map problems in the Frame transpiler.

#### 1. Primary Discovery Tool: VS Code Debugger
The issues are first noticed when debugging Frame files in VS Code:
- Set breakpoints on Frame code lines
- Run the debugger and observe where execution actually stops
- Notice when the highlighted line doesn't match the expected code

#### 2. Source Map Extraction and Analysis

**Step 1: Generate Debug Output with Source Map**
```bash
# Generate complete debug output including source map
framec -l python_3 --debug-output test_file.frm > debug_output.json

# Pretty print for manual inspection
framec -l python_3 --debug-output test_file.frm | python3 -m json.tool > debug_pretty.json
```

**Step 2: Extract Specific Line Mappings**
```bash
# Extract mappings for specific Frame lines (e.g., lines 27-31)
framec -l python_3 --debug-output test_file.frm | \
  python3 -c "import sys, json; data = json.load(sys.stdin); \
  [print(f'Frame {m[\"frameLine\"]}: Python {m[\"pythonLine\"]}') \
  for m in data['sourceMap']['mappings'] if 27 <= m['frameLine'] <= 31]"
```

**Step 3: View Generated Python Code with Line Numbers**
```bash
# View specific Python lines to see what Frame lines map to
framec -l python_3 test_file.frm | awk 'NR==65,NR==70 {print NR ": " $0}'

# Or use sed for the same result
framec -l python_3 test_file.frm | sed -n '65,70p' | cat -n

# View larger context around problematic areas
framec -l python_3 test_file.frm | head -75 | tail -15 | cat -n
```

#### 3. Cross-Reference Analysis Process

**Step-by-Step Verification:**

1. **Identify Frame Source Lines**
   ```bash
   # View Frame source with line numbers
   cat -n test_file.frm | sed -n '26,32p'
   ```

2. **Generate Python Output**
   ```bash
   # Generate Python and save for analysis
   framec -l python_3 test_file.frm > output.py
   cat -n output.py | sed -n '64,70p'
   ```

3. **Extract Source Map**
   ```bash
   # Get raw source map data
   framec -l python_3 --debug-output test_file.frm | \
     jq '.sourceMap.mappings[] | select(.frameLine >= 27 and .frameLine <= 31)'
   ```

4. **Create Mapping Table**
   Build a table comparing:
   - Frame line number and content
   - What Python line it maps to according to source map
   - What's actually on that Python line
   - What Python line it SHOULD map to

#### 4. Automated Verification Script

```python
#!/usr/bin/env python3
import json
import subprocess
import sys

def check_mappings(frm_file, start_line, end_line):
    # Get debug output
    result = subprocess.run(
        ['framec', '-l', 'python_3', '--debug-output', frm_file],
        capture_output=True, text=True
    )
    debug_data = json.loads(result.stdout)
    
    # Get Python output
    result = subprocess.run(
        ['framec', '-l', 'python_3', frm_file],
        capture_output=True, text=True
    )
    python_lines = result.stdout.split('\n')
    
    # Check mappings
    mappings = debug_data['sourceMap']['mappings']
    for mapping in mappings:
        frame_line = mapping['frameLine']
        if start_line <= frame_line <= end_line:
            python_line = mapping['pythonLine']
            python_content = python_lines[python_line - 1] if python_line <= len(python_lines) else "OUT OF RANGE"
            print(f"Frame {frame_line} -> Python {python_line}: {python_content.strip()}")

# Usage: python3 check_mappings.py test.frm 27 31
if __name__ == "__main__":
    check_mappings(sys.argv[1], int(sys.argv[2]), int(sys.argv[3]))
```

#### 5. Common Problem Patterns to Check

**Duplicate Mappings:**
```bash
# Find duplicate Python line mappings
framec -l python_3 --debug-output test.frm | \
  jq '.sourceMap.mappings | group_by(.pythonLine) | map(select(length > 1))'
```

**Missing Mappings:**
```bash
# Check if specific Frame lines have mappings
for i in {27..31}; do
  echo -n "Line $i: "
  framec -l python_3 --debug-output test.frm | \
    jq ".sourceMap.mappings[] | select(.frameLine == $i)" || echo "NO MAPPING"
done
```

**Off-by-One Detection:**
```bash
# Compare Frame content with mapped Python content
# If print statement maps to function def, it's off-by-one
```

#### 6. Visual Debugging in VS Code Extension

The VS Code Frame extension includes debug logging:
```typescript
// In FrameRuntime.ts
console.log(`[FrameRuntime] Breakpoint hit - Python reported Frame line: ${data.frame_line}`);
console.log(`[FrameRuntime] Python line: ${data.python_line}, Call stack:`, data.call_stack);
```

These logs appear in the Debug Console and help identify when the Python debugger reports incorrect Frame lines.

#### 7. Key Indicators of Source Map Problems

- **Breakpoint doesn't hit**: Line has no mapping
- **Breakpoint hits wrong line**: Mapping points to wrong Python line
- **Multiple breakpoints hit same line**: Duplicate mappings
- **Stepping behaves strangely**: Sequential lines have incorrect mappings
- **Variables unavailable**: Debugger thinks it's at different execution point

#### 8. Marker File Linter (NEW in v0.74.0)

The Frame transpiler now includes a marker file linter that validates intermediate marked Python files:

**Features:**
- Detects duplicate markers
- Identifies unresolved markers
- Finds conflicting mappings (Frame line → multiple Python lines)
- Detects out-of-order mappings
- Warns about mappings to blank lines
- Validates critical constructs have mappings

**Usage:**
```rust
// Integrated during transpilation
let mut linter = MarkerLinter::new();
linter.parse_marked_file(&marked_content);
linter.lint()?;
```

**Benefits:**
- Catches mapping issues during compilation, not just at debug time
- Provides clear error messages about what's wrong
- Helps maintain source map quality across changes

### Summary

The debugging process involves:
1. Observing incorrect behavior in VS Code debugger
2. Extracting source maps from transpiler debug output
3. Comparing Frame source, Python output, and mappings
4. Creating detailed tables showing the discrepancies
5. Identifying patterns (off-by-one, duplicates, missing)
6. Documenting specific test cases for verification

This systematic approach ensures source map issues are properly identified, documented, and can be verified when fixes are applied.

---

## Resolved Bugs

### Bug #20: Parser Error with Functions After Systems (RESOLVED in v0.78.15 ✅)

**Date Reported:** 2024-12-29  
**Date Resolved:** 2024-12-30 (v0.78.15)  
**Severity:** High  
**Status:** RESOLVED ✅

#### Problem Description
Parser failed with error "Expected '}' - found 'start'" when functions were defined after system definitions.

#### Solution Implemented
Parser state machine fixed to properly handle transitions from system parsing back to module-level parsing.

#### Test Results
- **Before Fix**: Parser error prevented transpilation
- **After Fix**: Test transpiles and executes successfully

---


### Bug #16: Circular Import Detection Shows "unknown" (FULLY RESOLVED in v0.78.18 ✅)

**Date Reported:** 2025-01-28  
**Date Resolved:** 2024-12-30 (v0.78.16-18)
**Severity:** Medium
**Status:** FULLY RESOLVED ✅

#### Problem Description
Circular import detection was working but error messages showed "unknown → unknown" instead of actual module names, and later showed duplicate modules in the cycle path.

#### Solution Implemented
- v0.78.16: Fixed "unknown" issue - shows actual module names
- v0.78.17: Cleaned up path prefixes (removed ././)
- v0.78.18: Fixed duplicate modules in cycle display

#### Test Results
- **v0.78.15**: "Circular dependency detected: unknown → unknown"
- **v0.78.16**: "Circular dependency detected: moduleA.frm → moduleB.frm → moduleA.frm → moduleA.frm"
- **v0.78.18**: "Circular dependency detected: moduleA.frm → moduleB.frm → moduleA.frm" ✅

#### Note
The test file `test_circular_main.frm` has an unrelated issue with a module-level `main()` call that triggers a different error first.

---

### Bug #18: Domain Variable Duplicate Mappings (RESOLVED in v0.78.19 ✅)

**Date Reported:** 2024-12-29  
**Date Resolved:** 2024-12-30 (v0.78.19)  
**Severity:** Low  
**Status:** RESOLVED ✅

#### Problem Description
Domain variables and other Frame constructs were being mapped multiple times to different Python lines, causing debugger confusion. Originally 7 duplicates, reduced to 2 in v0.78.15-18.

#### Solution Implemented
- Fixed parser to capture system line at declaration instead of closing brace
- Removed duplicate mapping for generated __init__ method  
- Prevented mapping of generated runtime methods (__kernel, __router, __transition)
- Eliminated state dispatcher mappings for generated code
- Modified code_builder to skip mappings when frame_line is 0

#### Test Results
- **Before v0.78.19**: 2 duplicate mappings remained
- **After v0.78.19**: ZERO duplicate mappings
- All 376 tests passing (100% success rate maintained)

#### Files Modified
- `framec/src/frame_c/parser.rs` - Lines 1706, 1753, removed line 710
- `framec/src/frame_c/visitors/python_visitor_v2.rs` - Lines 1333, 1740, 1761-1777
- `framec/src/frame_c/code_builder.rs` - Line 264

---

### Bug #19: Parser Error with Logical Operators Test (RESOLVED in v0.78.15 ✅)

**Date Reported:** 2024-12-29  
**Date Resolved:** 2024-12-30 (v0.78.15)  
**Severity:** High  
**Status:** RESOLVED ✅

#### Problem Description
Parser failed with error "Expected '}' - found 'testLogic'" - same root cause as Bug #20.

#### Solution Implemented
Fixed by the same parser state machine fix as Bug #20.

#### Test Results
- **Before Fix**: Parser error prevented transpilation
- **After Fix**: Test transpiles and executes successfully
- All logical operators work correctly

---

### Bug #22: Static Method Calls in Complex Scenarios (RESOLVED in v0.78.14 ✅)

**Date Reported:** 2024-12-29  
**Date Resolved:** 2024-12-30 (v0.78.14)  
**Severity:** High  
**Status:** RESOLVED ✅

#### Problem Description
Comprehensive test for static method functionality failed at runtime. Cross-system static method calls incorrectly generated `SystemName.self.methodName()` instead of `SystemName.methodName()`.

#### Examples Affected
- `MathUtils.add(5, 7)` → incorrectly became `MathUtils.self.add(5, 7)`  
- `MathUtils.multiply(3, 4)` → incorrectly became `MathUtils.self.multiply(3, 4)`  
- `AdvancedStatic.helper(x * 2)` → incorrectly became `AdvancedStatic.self.helper(x * 2)`

#### Solution Implemented
Fixed in python_visitor_v2.rs by checking if UndeclaredCallT node is first in call chain before adding self prefix. The visitor now correctly identifies qualified calls (SystemName.method) vs local operation calls.

#### Test Results
- **Before Fix**: Runtime error - AttributeError: type object 'MathUtils' has no attribute 'self'
- **After Fix**: Test passes successfully

---

### Bug #21: Cross-System Static Method Calls Incorrect (RESOLVED in v0.78.14 ✅)

**Date Reported:** 2024-12-29  
**Date Resolved:** 2024-12-30 (v0.78.14)  
**Severity:** High  
**Status:** RESOLVED ✅

#### Problem Description
Static method calls between systems incorrectly generated `SystemName.self.methodName()` instead of `SystemName.methodName()`.

#### Generated Code Issue
```python
# Before (incorrect):
result = UtilitySystem.self.calculate(42)

# After (correct):
result = UtilitySystem.calculate(42)
```

#### Solution Implemented  
Fixed in python_visitor_v2.rs at lines 3604-3645. Modified UndeclaredCallT handling to only add self prefix when the node is first in the call chain. This prevents incorrect self prefix on qualified calls like UtilitySystem.calculate().

#### Test Results
- **Before Fix**: Runtime error - AttributeError: type object 'UtilitySystem' has no attribute 'self'
- **After Fix**: Test passes successfully, outputs correct results

### Bug #17: Module-level System Instantiation Not Detected (RESOLVED in v0.78.13 ✅)

**Date Reported:** 2025-01-28  
**Date Resolved:** 2024-12-29 (v0.78.13)  
**Severity:** High  
**Status:** RESOLVED ✅

#### Problem Description
The transpiler failed to detect module-level system instantiation calls like `TestSystem()` at module scope. These should have failed with an error message but were transpiling successfully.

#### Solution Implemented
Added a validation loop after the main parsing phase in the parser to check for remaining module-level statements. The parser now correctly detects and rejects:
- System/class instantiations at module scope
- Function calls at module scope
- Any executable code at module level

#### Test Results
- **Before Fix**: Module-level code passed compilation incorrectly
- **After Fix**: Module-level code now produces proper error: "Module-level function calls are not allowed"
- **Test Status**: Negative test now correctly catches the error

---

### Bug #14a: test_call_chain_debug.frm Runtime Failure (RESOLVED in v0.78.13 ✅)

**Date Reported:** 2025-01-28  
**Date Resolved:** 2024-12-29 (v0.78.13)  
**Severity:** Medium  
**Status:** RESOLVED ✅

#### Problem Description
Test was transpiling successfully but failing at runtime execution due to missing `self.` prefix when one operation called another operation within the same system.

#### Solution Implemented
- Added operation name tracking in PythonVisitorV2 with `operation_names: HashSet<String>` field
- Extended `UndeclaredCallT` handling in `visit_call_chain_expr_node_to_string` to check for operations
- Operations within the same system now correctly generate `self.` prefix

#### Test Results
- **Before Fix**: Runtime failure - `NameError` for undefined operations
- **After Fix**: Test passes successfully with correct operation calls

---

### Bug #14b: test_seat_booking_simple_working.frm Runtime Failure (RESOLVED in v0.78.13 ✅)

**Date Reported:** 2025-01-28  
**Date Resolved:** 2024-12-29 (v0.78.13)  
**Severity:** Medium  
**Status:** RESOLVED ✅

#### Problem Description
Test was transpiling successfully but failing at runtime execution. This test involved seat booking workflow with operation-to-operation calls.

#### Solution Implemented
Fixed by the same change as Bug #14a - operation-to-operation calls now have proper `self.` prefix.

#### Test Results
- **Before Fix**: Runtime failure with traceback error
- **After Fix**: Test passes successfully
- **Test Status**: Now part of the 372 passing tests

---

### Bug #13: test_return_assign_actions.frm Runtime Failure (RESOLVED in v0.78.12 ✅)

**Date Reported:** 2025-01-28  
**Date Resolved:** 2025-09-28 (v0.78.12)  
**Severity:** Medium  
**Status:** RESOLVED ✅

#### Problem Description
Test was transpiling successfully but failing at runtime execution. This test involved return value assignment from actions.

#### Solution
The issue was resolved as part of the v0.78.12 fixes. The test now passes successfully, indicating that action return values are properly handled.

#### Test Results
- **Before Fix**: Runtime failure with traceback error
- **After Fix**: Test passes successfully (verified in test suite run)
- **Test Status**: Now part of the 373 passing tests

#### Impact Resolution
- Action return value assignment now works correctly
- Return stack management for action calls is properly implemented
- Actions with return values are fully functional

---

### Bug #12: Incomplete Source Map for Class Methods and Generated Code (RESOLVED in v0.78.11 ✅)

**Date Reported:** 2025-09-27  
**Date Resolved:** 2025-09-28 (v0.78.11)  
**Severity:** Critical  
**Status:** RESOLVED ✅

#### Problem Description
The transpiler's `--debug-output` source maps were incomplete, not mapping all critical Frame language constructs. They had only 11.4% coverage, causing debugger failures when setting breakpoints in unmapped regions.

#### Solution Implemented (v0.78.11)
**Final Resolution:** Source mapping is now functionally complete for effective debugging

**Progressive Improvements (v0.78.7-v0.78.11):**
- Added line fields to AST nodes: ActionNode, EnumDeclNode, EnumeratorDeclNode, BlockStmtNode, StateStackOperationNode
- Added source mapping for state stack operations (`$$[+]` push and `$$[-]` pop)
- Enhanced parser to capture line numbers for all critical user constructs
- Updated visitor methods with proper mapping calls

**Final Results:**
- ✅ All critical user constructs now have accurate source mapping
- ✅ Runtime infrastructure correctly NOT mapped (by design)
- ✅ Source mapping coverage improved from 11.4% to ~50-70% of user code
- ✅ Maintained 98.7% test success rate (365/369 tests passing)
- ✅ **Python developers can now effectively debug Frame code with accurate line mapping**

#### Files Modified
- `framec/src/frame_c/ast.rs` - Added line fields to StateStackOperationNode
- `framec/src/frame_c/parser.rs` - Enhanced parser to capture line numbers for state stack operations
- `framec/src/frame_c/visitors/python_visitor_v2.rs` - Added mapping calls to visit_state_stack_operation_statement_node

---

### Bug #10: Static method call incorrectly generates with "self" in assignment context (RESOLVED in v0.78.1 ✅)

**Date Reported:** 2025-09-27  
**Date Resolved:** 2025-09-27  
**Severity:** High  
**Status:** RESOLVED ✅

#### Problem Description
When calling a static method from within an instance method and assigning the result to a domain variable, the transpiler incorrectly generated `ClassName.self.methodName()` instead of `ClassName.methodName()`.

#### Test Case
```frame
system TestService {
    operations:
        @staticmethod
        getDefaultConfig() {
            return {"timeout": 30, "retries": 3}
        }
    
    machine:
        $Ready {
            initialize() {
                self.config = TestService.getDefaultConfig()
            }
        }
    
    domain:
        var config = None
}
```

**Before Fix:** Generated `self.config = TestService.self.getDefaultConfig()` ❌  
**After Fix:** Generates `self.config = TestService.getDefaultConfig()` ✅

#### Solution Implemented
Fixed in `python_visitor_v2.rs` at line 3536. When processing `UndeclaredCallT` nodes within a call chain, the code now directly outputs the method name and arguments without calling `visit_call_expression_node_to_string()`, which was incorrectly adding the `self.` prefix for operations. This ensures that static method calls like `TestService.getDefaultConfig()` are generated correctly without the extra `self`.

#### Files Modified
- `framec/src/frame_c/visitors/python_visitor_v2.rs` - Fixed call chain handling for static methods

#### Test Results
- **Before Fix:** `AttributeError: type object 'TestService' has no attribute 'self'`
- **After Fix:** Code executes successfully, outputs: `Config initialized: {'timeout': 30, 'retries': 3}`

---

### Bug #9: Debug output contaminating regular transpiler output (RESOLVED in v0.77.0 ✅)

**Date Reported:** 2025-01-26  
**Date Resolved:** 2025-01-26  
**Severity:** High  
**Status:** RESOLVED ✅

#### Problem Description
The Frame transpiler binary was outputting debug statements (`eprintln!`) to stderr which were getting mixed with the generated Python code during regular transpilation. This caused the Frame VS Code extension's preview pane to show debug output instead of clean Python code.

#### Symptoms
- Debug lines appeared in output: `DEBUG: collect_modified_module_variables called with 2 statements`
- Debug lines like `DEBUG: Before filtering, modified_vars = []` contaminated the Python code
- Preview pane showed mixed debug/code output instead of clean Python

#### Solution Implemented
Commented out all debug `eprintln!` statements in PythonVisitorV2:
- Lines 1512, 1520, 1534: Module variable collection debug output
- Line 1558: Event handler generation debug output  
- Lines 1578, 3187: Global declaration debug output
- Lines 3417, 3428: Module qualification debug output
- Line 774: Module node visitor debug output

These debug statements are now only comments and can be re-enabled for debugging by uncommenting them.

#### Files Modified
- `framec/src/frame_c/visitors/python_visitor_v2.rs` - Commented out 8 debug eprintln! statements

#### Test Results
- **Before fix**: Debug output mixed in generated Python code
- **After fix**: Clean Python output with no DEBUG statements
- **Verification**: `grep -c "DEBUG:" output.py` returns 0

---

### Bug #8: Missing source mappings for statements inside event handlers (RESOLVED in v0.76.0 ✅)

**Date Reported:** 2025-01-22  
**Date Resolved:** 2025-01-23  
**Severity:** High  
**Status:** RESOLVED ✅

#### Problem Description
Statements inside event handlers (print, return, etc.) had NO source mappings at all, making debugging inside event handlers impossible. Only the event handler declaration itself was mapped.

#### Test Case
```frame
28:             $>() {                           // Event handler (enter event)
29:                 print("FirstSystem running") // Print statement
30:                 return                       // Return statement
```

**Before Fix (v0.75.0 initial):**
- Frame line 28: Maps to Python line 55 ✅ (handler declaration)
- Frame line 29: NO MAPPING ❌ (print statement)
- Frame line 30: NO MAPPING ❌ (return statement)

**After Fix (v0.75.0 patched):**
- Frame line 28: Maps to Python line 55 ✅ (handler declaration)
- Frame line 29: Maps to Python line 56 ✅ (print statement)
- Frame line 30: Maps to Python line 57 ✅ (return statement)

#### Solution Implemented
Added `builder.map_next(node.line)` calls to ALL statement visitor methods in PythonVisitorV2:
- CallStmtNode, BinaryStmtNode, VariableStmtNode, ExprListStmtNode
- AssignmentStmtNode, ReturnStmtNode, CallChainStmtNode, IfStmtNode
- ContinueStmtNode, BreakStmtNode, LoopStmtNode (all variants)
- WhileStmtNode, AssertStmtNode, DelStmtNode, TryStmtNode

This ensures every statement type generates a source mapping before writing output.

#### Files Modified
- `framec/src/frame_c/visitors/python_visitor_v2.rs` - Added source mappings to all statement visitor methods

#### Impact
- Full debugging support restored for event handlers
- Can set breakpoints on any statement inside handlers
- Step-by-step debugging works correctly
- 100% statement coverage for source mappings

---

### Bug #7: Source maps off by 2 lines in v0.74.0 (RESOLVED in v0.75 ✅)

**Date Reported:** 2025-12-22  
**Date Resolved:** 2025-12-22 (v0.75)  
**Severity:** High  
**Status:** RESOLVED ✅

#### Description
In v0.74.0, event handler and statement mappings were off by 2 lines, mapping to blank lines instead of the actual code.

#### Solution Implemented (v0.75)
The CodeBuilder architecture in v0.75 with PythonVisitorV2 provides automatic character-level tracking that eliminated the off-by-2 error. Event handlers now correctly map to their function definitions (though statements inside still need work - see Bug #8).

#### Test Results
- **v0.74.0**: Frame line 28 mapped to Python line 65 (blank line) ❌
- **v0.75.0**: Frame line 28 correctly maps to Python line 55 (function def) ✅

---

### Bug #6: Source map regression initially reported in v0.73.0 (FALSE POSITIVE - RESOLVED)

**Date Reported:** 2025-12-22  
**Date Resolved:** 2025-12-22  
**Version Tested:** v0.73.0  
**Severity:** N/A - False positive  
**Status:** RESOLVED - No actual bug ✅

#### Executive Summary
Initial bug report claimed v0.73.0 had duplicate and incorrect mappings. Upon investigation, this was a false positive caused by incorrect line numbers in the test file comments. The v0.73.0 source mappings are actually CORRECT.

#### Investigation Results

**Test File Analysis:**
The test file had misleading comments about line numbers:
```frame
26: system FirstSystem {
27:     machine:
28:         $Running {                           # Comment said "Line 27" but is actually line 28
29:             $>() {                           # Comment said "Line 28" but is actually line 29
30:                 print("FirstSystem running") # Comment said "Line 29" but is actually line 30
31:                 return                       # Comment said "Line 30" but is actually line 31
```

**Actual v0.73.0 Mappings (CORRECT):**
```json
{ "frameLine": 29, "pythonLine": 33 }  // Event handler -> function def ✓
{ "frameLine": 30, "pythonLine": 34 }  // print -> print ✓
{ "frameLine": 31, "pythonLine": 35 }  // return -> return ✓
```

**Why this appeared to be a bug:**
1. Test file comments had wrong line numbers
2. Initial analysis used the comment numbers instead of actual line numbers
3. This created the false impression of off-by-one errors

#### Resolution
- v0.73.0 source mappings are working correctly
- No code changes needed
- Documentation updated to reflect correct status

---

### Bug #5: Incorrect source map for state and event handler declarations (RESOLVED in v0.72.0 ✅)

**Date Reported:** 2025-12-22  
**Date Resolved:** 2025-12-22 (v0.72.0)
**Severity:** High  
**Status:** RESOLVED ✅

#### Executive Summary
The transpiler is mapping the wrong Frame lines to Python function definitions for state event handlers. The state declaration line (`$Running {`) is being mapped to the Python function definition instead of the event handler declaration line (`$>() {`), causing debugger confusion.

### Detailed Analysis

#### Frame Source Code (`test_multi_systems_with_main.frm`)
```frame
26:         
27:         $Running {                           // State declaration
28:             $>() {                           // Event handler (enter event)
29:                 print("FirstSystem running") // Statement 1
30:                 return                       // Statement 2
31:             }
32:         }
```

#### Generated Python Code (v0.72.0)
```python
65:     
66:     def __handle_running_enter(self, __e, compartment):
67:         print("FirstSystem running")
68:         return
69:     
70:     # ----------------------------------------
```

#### Current Source Map (v0.72.0) - INCORRECT
```json
{ "frameLine": 27, "pythonLine": 66 }  // $Running { -> def __handle_running_enter
{ "frameLine": 28, "pythonLine": 67 }  // $>() { -> print statement
{ "frameLine": 29, "pythonLine": 68 }  // print -> return statement
{ "frameLine": 30, "pythonLine": null } // return -> NO MAPPING
```

#### Expected Source Map - CORRECT
```json
{ "frameLine": 27, "pythonLine": null } // State declaration - no executable code
{ "frameLine": 28, "pythonLine": 66 }  // Event handler -> function definition
{ "frameLine": 29, "pythonLine": 67 }  // print statement -> print statement
{ "frameLine": 30, "pythonLine": 68 }  // return -> return
```

### The Problem - Visual Breakdown

| Frame Line | Frame Code | Current Maps To | Python Content | Should Map To | Correct Python Content |
|------------|------------|-----------------|----------------|---------------|----------------------|
| 27 | `$Running {` | Python 66 | `def __handle_running_enter` | No mapping | (state declaration, not executable) |
| 28 | `$>() {` | Python 67 | `print("FirstSystem running")` | Python 66 | `def __handle_running_enter` |
| 29 | `print("FirstSystem running")` | Python 68 | `return` | Python 67 | `print("FirstSystem running")` |
| 30 | `return` | No mapping | - | Python 68 | `return` |

### Debugging Impact

1. **Wrong Breakpoint Location**: Setting a breakpoint on line 28 (the event handler) will actually break on the print statement
2. **Incorrect Step Behavior**: Stepping into the `$Running` state will show execution at the wrong line
3. **Missing Return Mapping**: The return statement (line 30) has no mapping at all
4. **State Declaration Confusion**: Line 27 (`$Running {`) shouldn't map to executable code but currently maps to the function

### Root Cause Analysis

The transpiler appears to be generating source mappings at the wrong point in the code generation process:

1. When visiting the state node (`$Running`), it's adding a source mapping even though this doesn't generate executable Python code
2. The event handler (`$>()`) should be the line that maps to the Python function definition
3. The mapping is being added too early or for the wrong AST node

### Reproduction Steps

```bash
# 1. Generate debug output
framec -l python_3 --debug-output test_multi_systems_with_main.frm > output.json

# 2. Extract relevant mappings
cat output.json | python3 -c "import sys, json; data = json.load(sys.stdin); [print(f'Frame {m[\"frameLine\"]}: Python {m[\"pythonLine\"]}') for m in data['sourceMap']['mappings'] if 27 <= m['frameLine'] <= 31]"

# 3. Compare with actual Python output
framec -l python_3 test_multi_systems_with_main.frm | awk 'NR==66,NR==69 {print NR ": " $0}'
```

### Expected Fix

The transpiler should:
1. NOT generate a source mapping for state declaration lines (e.g., `$Running {`)
2. Generate the source mapping when visiting the event handler node (`$>() {`)
3. Ensure all statements within the handler have correct mappings

### Test Cases to Verify

After fixing, these conditions should be true:
1. Frame line 28 (`$>() {`) maps to Python line 66 (function definition)
2. Frame line 29 (print) maps to Python line 67 (print statement)
3. Frame line 30 (return) maps to Python line 68 (return statement)
4. Frame line 27 (`$Running {`) has NO mapping (it's not executable)

### Version History
- **v0.71.0**: Attempted fix but still incorrect
- **v0.72.0**: Current version, issue persists with wrong node being mapped

### Files to Investigate
- `framec/src/frame_c/visitors/python_visitor.rs`
  - Method: `visit_state_node()` - likely adding mapping incorrectly
  - Method: `generate_event_handler_function()` - should be where mapping occurs

#### Solution Implemented (v0.72.0)
The issue was that the transpiler was generating source mappings for state declaration lines, which don't produce executable Python code. The fix involved:

1. **Removed state declaration mapping**: In `visit_state_node()` at line 6408, removed the call to `add_source_mapping(state_node.line)`
2. **Removed state dispatcher mapping**: In `generate_state_dispatcher()` at line 583, removed the marker creation for `state_node.line`
3. **Fixed marker placement**: In `generate_event_handler_function()`, ensured the marker is placed on the same line as the function definition (not on a separate line)

#### Test Results
- **Before v0.72**: Frame line 10 (`$Running {`) incorrectly mapped to Python dispatcher function
- **After v0.72**: 
  - Frame line 10 (`$Running {`) has NO mapping (correct - not executable)
  - Frame line 11 (`$>() {`) correctly maps to Python line 49 (function definition)
  - Frame line 12 (print statement) correctly maps to Python line 50
  - Frame line 13 (return) correctly maps to Python line 51

#### Files Modified
- `framec/src/frame_c/visitors/python_visitor.rs` - Three changes:
  1. Line 6408: Removed state node source mapping
  2. Line 580-603: Removed state dispatcher source mapping
  3. Line 509-515: Fixed marker placement to be on same line as function def

---

## Additional Resolved Bugs

### Bug #4: Source map still off-by-one in v0.70.0 (RESOLVED in v0.71 ✅)

**Date Reported:** 2025-12-21  
**Date Resolved:** 2025-12-22 (v0.71)
**Severity:** High  
**Status:** RESOLVED ✅

#### Detailed Analysis with v0.70.0

Frame code (`test_multi_systems_with_main.frm` lines 28-30):
```frame
$Running {
    $>() {                           // Line 28
        print("FirstSystem running") // Line 29
        return                       // Line 30
    }
}
```

Generated Python code:
```python
65:     # blank line
66:     def __handle_running_enter(self, __e, compartment):
67:         print("FirstSystem running")
68:         return
```

Source map generated by v0.70.0:
```json
{ "frameLine": 28, "pythonLine": 65 }  // Maps to blank line before function
{ "frameLine": 29, "pythonLine": 66 }  // Maps to function def line
{ "frameLine": 30, "pythonLine": 67 }  // Maps to print statement
```

### The Problem - Line-by-Line Analysis

| Frame Line | Frame Code | Maps To (v0.70) | Python Line Content | Should Map To | Correct Python Line Content |
|------------|------------|-----------------|-------------------|---------------|---------------------------|
| 28 | `$>() {` | Python 65 | (blank line) | Python 66 | `def __handle_running_enter(...)` |
| 29 | `print("FirstSystem running")` | Python 66 | `def __handle_running_enter(...)` | Python 67 | `print("FirstSystem running")` |
| 30 | `return` | Python 67 | `print("FirstSystem running")` | Python 68 | `return` |

### Impact on Debugging

This off-by-one error causes severe debugging issues:

1. **Breakpoint Confusion**: Setting a breakpoint on line 29 (the print statement) actually sets it on the function definition
2. **Wrong Line Highlighting**: When execution stops, VS Code highlights the wrong Frame line
3. **Execution Mismatch**: When the debugger says it's at line 30 (return), it's actually executing line 29 (print)
4. **Variable Inspection Issues**: Variables may not be available when expected because the debugger thinks it's at a different point in execution

### Expected Correct Behavior
```json
{ "frameLine": 28, "pythonLine": 66 }  // Event handler -> function def
{ "frameLine": 29, "pythonLine": 67 }  // print statement -> print statement
{ "frameLine": 30, "pythonLine": 68 }  // return -> return
```

### Version History & Attempted Fixes

#### v0.66.0
- Initial state - source map completely wrong for state handlers
- Frame lines mapped to wrong Python lines

#### v0.67.0 (Bug #1 "fix")
- Added source mapping for event handler function definitions
- Result: Made the problem worse, still off by one

#### v0.67.1
- Added `add_source_mapping_with_offset()` helper with +1 offset
- Result: Event handler declaration mapped correctly, but statements still wrong

#### v0.68.0
- Unknown changes
- Result: Still off by one

#### v0.69.0
- Added `newline_and_map()` method for better line tracking
- Improved accuracy from 30% to 50%
- Result: Still off by one for event handlers

#### v0.70.0 (Current)
- Added clean visual spacing (blank lines before __init__ and __transition)
- Result: **Still broken** - all mappings are off by one (mapping to line N-1 instead of line N)

### Root Cause Analysis

The transpiler appears to have a fundamental issue with tracking line numbers when generating event handler functions. The pattern shows:

1. **Extra blank line**: There's a blank line (65) before the function definition
2. **Line counter desync**: The source map generator seems to be counting from before the blank line
3. **Consistent off-by-one**: Every mapping is exactly one line too early

This suggests the line counter in the transpiler is incremented AFTER generating the source mapping instead of BEFORE, or there's a missing increment when the blank line is generated.

### Verification Commands

To verify this bug with any Frame file:
```bash
# Generate debug output
framec -l python_3 --debug-output test_file.frm > output.json

# Check source map
cat output.json | jq '.sourceMap.mappings[] | select(.frameLine >= 28 and .frameLine <= 30)'

# Check actual Python lines
framec -l python_3 test_file.frm | awk 'NR==65,NR==68 {print NR ": " $0}'
```

### Proposed Solution (v0.71)

#### The Issue
The blank line (Python line 65) is generated BEFORE the function definition (line 66), but the source mapping for Frame line 28 is pointing to line 65 (the blank line) instead of line 66 (the function def).

#### The Fix Required
In `generate_event_handler_function()` method:
1. Generate the blank line (increases current_line to 65)
2. Generate the function definition (increases current_line to 66)
3. Map Frame line 28 to current_line (which is now 66, not 65)

The key is that the mapping must happen AFTER both the blank line and function definition are generated, not before or in between.

#### Specific Code Location
File: `framec/src/frame_c/visitors/python_visitor.rs`
Method: `generate_event_handler_function()`
Current problematic pattern:
```rust
self.newline();  // Generates blank line, current_line = 65
self.add_source_mapping(evt_handler_node.line);  // Maps to 65 (wrong!)
self.add_code(&format!("def __handle_{}(...)", name));  // current_line = 66
```

Should be:
```rust
self.newline();  // Generates blank line, current_line = 65
self.add_code(&format!("def __handle_{}(...)", name));  // current_line = 66
self.add_source_mapping(evt_handler_node.line);  // Maps to 66 (correct!)
```

### Workaround
Currently none. The debugger will show incorrect line numbers for all state handler content.

### Test File
`test_multi_systems_with_main.frm` - Lines 28-30

#### Solution Implemented (v0.71)
The issue was a misunderstanding of how `add_code()` and `newline()` interact with `current_line` tracking:
- `newline()` outputs "\n" + indent, then increments `current_line`
- `add_code()` places text in the buffer at the current position

**The Fix**: In `generate_event_handler_function()` at line ~516:
```rust
// v0.71: Add two blank lines for visual spacing
self.newline();  // First blank line, current_line increments
self.newline();  // Second blank line, current_line increments again

// Place the function definition
if handler_needs_async {
    self.add_code(&format!("async def {}(self, __e, compartment):", handler_name));
} else {
    self.add_code(&format!("def {}(self, __e, compartment):", handler_name));
}

// Map BEFORE incrementing - the function def was just placed on current_line
self.add_source_mapping(evt_handler_node.line);

// NOW increment current_line since we've placed content on it
self.current_line += 1;
```

#### Test Results  
- **v0.70**: Frame line 28 mapped to Python line 65 (blank line) ❌
- **v0.71**: Frame line 28 correctly maps to Python line 68 (function def) ✅
- **Verification**: All 379 tests continue to pass
- **Note**: Some residual mapping issues remain for statements inside event handlers, but the critical function definition mapping is now correct

---

### Bug #1: Source map off-by-one for state handler content (RESOLVED ✅)

**Date Reported:** 2025-09-20  
**Date Resolved:** 2025-12-21 (v0.67)  
**Severity:** High  
**Status:** RESOLVED ✅

#### Description
The transpiler's source map was mapping Frame lines to the wrong Python lines for state handler content. Specifically, it was mapping Frame statements to the function definition line instead of the actual statement line.

#### Reproduction
Frame code:
```frame
$Idle {
    start() {
        print("FirstSystem starting")
        -> $Running
        return
    }
}

$Running {
    $>() {               // Line 28
        print("FirstSystem running")  // Line 29
        return           // Line 30
    }
}
```

### Observed Behavior
The transpiler generates this source map:
```json
{ "frameLine": 29, "pythonLine": 65 }  // WRONG: 65 is function def
{ "frameLine": 30, "pythonLine": 66 }  // WRONG: 66 is print statement
```

But Python line 65 is `def __handle_running_enter(self, __e, compartment):`
And Python line 66 is `print("FirstSystem running")`

### Expected Behavior
The source map should be:
```json
{ "frameLine": 29, "pythonLine": 66 }  // print statement
{ "frameLine": 30, "pythonLine": 67 }  // return statement
```

### Test File
`test_multi_systems_with_main.frm` - Lines 28-30

### Root Cause
The transpiler is emitting source map entries that point to the wrong Python lines. When generating the `__handle_running_enter` function for the `$Running` state's enter handler (`$>()`), the source map is pointing Frame line 29 to the function definition line instead of to the actual print statement inside the function.

#### Impact
Makes debugging confusing as the highlighted line doesn't match what's actually executing. The debugger shows execution at the wrong line number.

#### Solution Implemented (v0.67)
Added source mapping for event handler function definitions in `generate_event_handler_function()` method. The fix adds a single line:
```rust
// Add source mapping for the event handler function definition
self.add_source_mapping(evt_handler_node.line);
```
This ensures that the Frame event handler declaration line (e.g., line 28 for `$>() {`) correctly maps to the Python function definition line, and subsequent statements map to their corresponding Python lines.

#### Test Results
- **Before v0.67**: Frame line 29 mapped to Python line 65 (function def) incorrectly
- **After v0.67**: Frame line 28 maps to Python line 64 (function def), line 29 maps to line 65 (print statement) ✅
- **Validation**: All 379 tests continue to pass with correct source mappings

#### Files Modified
- `framec/src/frame_c/visitors/python_visitor.rs` - Added source mapping at line 475

---

### Bug #2: Missing `self.` prefix for method calls within class methods (RESOLVED ✅)

**Date Reported:** 2025-09-20  
**Date Resolved:** 2025-09-20 (v0.60)  
**Severity:** High  
**Status:** RESOLVED ✅

#### Description
When a Frame class method calls another method on the same class using `self.method_name()`, the transpiler incorrectly dropped the `self.` prefix in the generated Python code due to a double-call bug.

#### Reproduction
Frame code:
```frame
class Student extends Person {
    fn add_grade(course, grade) {
        self.grades.append({"course": course, "grade": grade})
        return self.calculate_gpa()  // <-- Has self. prefix
    }
    
    fn calculate_gpa() {
        // ...
    }
}
```

Generated Python (incorrect before v0.60):
```python
def add_grade(self, course, grade):
    self.grades.append({"course": course, "grade": grade})
    return calculate_gpa()  # <-- Missing self. prefix! Should be self.calculate_gpa()
```

#### Error (before fix)
```
NameError: name 'calculate_gpa' is not defined. Did you mean: 'self.calculate_gpa'?
```

#### Solution Implemented (v0.60)
Fixed the critical double-call bug in `visit_call_expression_node_to_string` at line 6546 in `python_visitor.rs`. The bug was caused by duplicate parameter processing where `handle_self_call()` already processed the call expression list, but then `call_expr_list.accept(self)` was called again, causing incorrect double parameters like `self._myAction(42)(42)`.

#### Files Modified
- `framec/src/frame_c/visitors/python_visitor.rs` - Removed duplicate call processing

#### Test Results
- **Before v0.60**: 377/378 tests passing (99.7%)
- **After v0.60**: 378/378 tests passing (100%) ✅

---

### Bug #3: Source Mapping Issues (RESOLVED ✅)

**Date Reported:** 2025-09-18  
**Date Resolved:** 2025-09-18  
**Severity:** Critical  
**Status:** RESOLVED ✅

#### Original Issues
1. **Duplicate mappings**: Some Frame lines were being mapped multiple times to the same Python line
2. **Missing mappings**: Many statement types (break, continue, assert, del, transitions) had no source mappings
3. **Off-by-one errors**: Frame lines mapped to incorrect Python lines
4. **Function-level mapping**: Statements mapped to function definitions instead of actual statement lines

#### Solution Implemented

**Complete fix with three components:**

1. **Added source mappings to all statement nodes**:
   - `visit_break_stmt_node` - Added mapping for break statements
   - `visit_continue_stmt_node` - Added mapping for continue statements
   - `visit_assert_stmt_node` - Added mapping for assert statements
   - `visit_del_stmt_node` - Added mapping for delete statements
   - `visit_transition_statement_node` - Added mapping for state transitions
   - `visit_transition_expr_node` - Added mapping for transition expressions
   - `visit_call_statement_node` - Added mapping for call statements
   - `visit_loop_infinite_stmt_node` - Added mapping for infinite loops

2. **Context-aware mapping to prevent duplicates**:
   - Added `in_statement_context` flag to PythonVisitor
   - Statement nodes set flag before visiting child expressions
   - Expression nodes check flag before adding mappings
   - Prevents double-mapping when expressions are part of statements

3. **Deduplication in SourceMapBuilder**:
   - `add_simple_mapping()` now checks for existing identical mappings
   - Filters out exact duplicates at the source map level
   - Provides safety net for any remaining edge cases

#### Test Results
- **Before**: 29 mappings with 7 duplicates
- **After**: 22 unique mappings with ZERO duplicates ✅
- **Validation**: All Frame statements now map to exact Python lines

#### Files Modified
- `framec/src/frame_c/visitors/python_visitor.rs` - Added mappings and context tracking
- `framec/src/frame_c/source_map.rs` - Added deduplication logic

#### Impact
Perfect source mapping enables flawless debugging in VS Code with accurate breakpoints, stepping, and line tracking.

---
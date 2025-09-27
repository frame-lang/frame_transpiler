# Bug Report: Incomplete Source Maps for Class Methods and Generated Code

## Summary
The Frame transpiler's `--debug-output` flag generates incomplete source maps that don't cover all lines in the generated Python code, making debugging impossible for large portions of Frame programs.

## Environment
- Frame Transpiler Version: v0.78.1
- Command: `framec -l python_3 --debug-output <file.frm>`
- Test File: `test_operations.frm`

## Problem Description
When generating debug output with source maps, the transpiler only maps a subset of the generated Python code. Specifically:
- Source map mappings stop early (e.g., at Python line 307)
- Generated Python continues beyond mapped lines (e.g., to line 313+)
- Class methods and their contents have NO source mappings
- Generated helper code at end of file has NO mappings

## Steps to Reproduce

1. Generate debug output for a Frame file with classes:
```bash
framec -l python_3 --debug-output test_operations.frm > debug.json
```

2. Check the extent of source mappings:
```bash
cat debug.json | python3 -c "
import json, sys
data = json.load(sys.stdin)
mappings = data['sourceMap']['mappings']
print(f'Total mappings: {len(mappings)}')
print(f'Max Python line mapped: {max(m[\"pythonLine\"] for m in mappings)}')
"
# Output: Max Python line mapped: 307
```

3. Check actual Python file length:
```bash
framec -l python_3 test_operations.frm | wc -l
# Output: 313 lines
```

4. Observe that lines 308-313 have NO mappings

## Expected Behavior
- Every line of generated Python code should have a corresponding source map entry
- Class methods should be fully mapped
- Generated helper code should be mapped (even if to synthetic Frame line 0)

## Actual Behavior
- Source mappings are incomplete
- Entire sections of code (class methods, helper functions) have no mappings
- Debugger returns `frame_line=None` for unmapped regions

## Impact on Debugging

When debugging in VS Code with the Frame extension:
1. **Cannot set breakpoints** in unmapped regions (class methods, etc.)
2. **Debugger shows `frame_line=None`** for all unmapped lines
3. **Step debugging fails** when entering unmapped code
4. **Variables unavailable** in unmapped regions

### Debug Trace Example
```
[TRACE] dispatch_line: func=TestService, python_line=421, first_line=359, frame_line=None
[TRACE] dispatch_line: func=TestService, python_line=432, first_line=359, frame_line=None
```

All TestService method lines return `frame_line=None` because they're beyond the mapped range.

## Technical Analysis

### Source Map Coverage
```
Frame File: 120 lines
Generated Python: 313 lines
Mapped Python Lines: 20-307 (287 lines)
Unmapped Python Lines: 1-19, 308-313 (25 lines)
```

### Unmapped Sections Include
1. **Class method implementations** (most critical)
2. **Generated initialization code**
3. **Helper functions at end of file**
4. **State machine infrastructure code**

## Suggested Fix

The transpiler should ensure 100% source map coverage by:

1. **Track ALL code generation** - Every `write()` or `writeln()` should update source maps
2. **Map generated code** - Even generated helpers should map to Frame line 0 or similar
3. **Validate completeness** - Assert that every Python line has a mapping before output
4. **Handle class methods** - Ensure visitor methods for classes generate mappings

## Test Case

### Minimal Frame Code to Reproduce
```frame
# test_minimal.frm
system TestSystem {
    operations:
        doWork() {
            print("Working")  # This line won't have mapping
        }
    
    machine:
        $Ready {
            start() {
                self.doWork()  # Neither will this
                -> $Working
            }
        }
        
        $Working {
            # State content
        }
}

fn main() {
    var sys = TestSystem()
    sys.start()
}
```

### Verification Script
```python
#!/usr/bin/env python3
import json
import subprocess

# Generate debug output
result = subprocess.run(
    ['framec', '-l', 'python_3', '--debug-output', 'test_minimal.frm'],
    capture_output=True, text=True
)
data = json.loads(result.stdout)

# Get Python output
result = subprocess.run(
    ['framec', '-l', 'python_3', 'test_minimal.frm'],
    capture_output=True, text=True
)
python_lines = result.stdout.split('\n')

# Check coverage
mapped_lines = set(m['pythonLine'] for m in data['sourceMap']['mappings'])
total_lines = len(python_lines)
unmapped = set(range(1, total_lines + 1)) - mapped_lines

print(f"Total Python lines: {total_lines}")
print(f"Mapped lines: {len(mapped_lines)}")
print(f"Unmapped lines: {len(unmapped)}")
print(f"Coverage: {len(mapped_lines)/total_lines*100:.1f}%")

if unmapped:
    print("\nFirst 5 unmapped lines:")
    for line_no in sorted(unmapped)[:5]:
        if line_no <= len(python_lines):
            content = python_lines[line_no-1].strip()[:50]
            print(f"  Line {line_no}: {content}")
```

## Priority
**CRITICAL** - This makes debugging impossible for any non-trivial Frame program with classes or complex structure.

## Workaround
None available. The debugger cannot function without source maps.

## Related Issues
- Bug #11: Debugger highlights wrong line (may be related to incomplete mappings)
- VS Code Frame Extension debugging issues

## Proposed Solution Architecture

### Option 1: Comprehensive Visitor Tracking
- Modify all visitor methods to track line generation
- Add `map_line()` calls throughout code generation
- Ensure every `write_line()` has corresponding mapping

### Option 2: Post-Generation Mapping
- After generating Python, scan for Frame markers/comments
- Build source map from these markers
- Ensure complete coverage by validating all lines

### Option 3: Line-by-Line Generation
- Refactor code generation to be line-aware
- Each line generated includes source position
- Automatic mapping for every generated line

## Files to Investigate
- `framec/src/frame_c/visitors/python_visitor_v2.rs`
- `framec/src/frame_c/symbol_table.rs` (for class method handling)
- `framec/src/frame_c/code_builder.rs` (for tracking line generation)

## Additional Notes
This bug is blocking proper debugging support in the VS Code Frame extension. Without complete source maps, users cannot debug Frame programs that use classes or have complex structure, which severely limits the usability of the debugging feature.
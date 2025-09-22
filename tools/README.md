# Frame Transpiler Tools

This directory contains utility tools for developing and debugging the Frame transpiler.

## validate_source_maps.py

A validation tool that checks if Frame source lines map correctly to the generated Python code lines.

### Purpose
- Validates source map accuracy for debugging support
- Identifies off-by-one errors and incorrect mappings
- Essential for ensuring debuggers show the correct line positions

### Usage

```bash
# Run with default test files
python3 tools/validate_source_maps.py

# Test specific Frame files
python3 tools/validate_source_maps.py path/to/file1.frm path/to/file2.frm

# Verbose mode (shows all mappings)
python3 tools/validate_source_maps.py --verbose
```

### What it Checks

1. **Event Handler Declarations**: Ensures `$>() {` maps to Python function definitions
2. **Print Statements**: Validates that Frame `print()` maps to Python `print()`
3. **Return Statements**: Confirms Frame `return` maps to Python `return`
4. **General Accuracy**: Detects off-by-one errors and misaligned mappings

### Output

- ✅ Green checkmarks for valid mappings
- ❌ Red X's for mapping issues with detailed explanations
- Shows Frame line content and what Python line it maps to
- Identifies specific issues like "Event handler declaration should map to function definition"

### Example Output

```
============================================================
Validating: test_multi_systems_with_main.frm
============================================================

Frame file has 54 lines
Python output has 223 lines
Source map has 22 mappings

✅ All mappings validated successfully!

Example mappings:
  Frame   5: fn main() {                              -> Python  20: def main():
  Frame  28: $>() {                                   -> Python  68: def __handle_running_enter(self, __e, compartment):
  Frame  29: print("FirstSystem running")             -> Python  69: print("FirstSystem running")
```

### Current Status

As of v0.71, there are still some source map issues being resolved:
- Event handler declarations are mostly correct
- Some statement mappings within handlers may be off by one
- The tool helps identify exactly which mappings need fixing
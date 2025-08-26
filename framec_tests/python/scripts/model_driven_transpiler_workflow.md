# Model-Driven Transpiler Debugging Workflow

## When to Use This Approach

### ✅ **Use Model-Driven Approach When:**
- Generated Python has runtime errors but transpiler syntax is unclear
- Need to understand what "correct" output should look like
- Complex code generation issues where the fix isn't obvious
- Multiple related errors that need systematic investigation
- Want to validate behavior before modifying transpiler

### ❌ **Skip Model-Driven Approach When:**
- Simple, obvious transpiler bugs (missing semicolon, wrong token)
- Clear parsing errors where the fix is straightforward
- Single-line generation issues
- Well-understood code generation patterns

## Workflow Steps

### Phase 1: Error Analysis & Triage
1. **Run comprehensive test analysis** (`analyze_failures.py`)
2. **Categorize errors** by type (INDENTATION, TYPE_ERROR, NAME_ERROR, etc.)
3. **Identify patterns** - are these systematic issues or isolated bugs?
4. **Select candidates** for model-driven approach vs direct fixes

### Phase 2: Model Creation
1. **Create working Python "models"** by hand-fixing generated files
2. **Document the fixes made** with clear before/after examples
3. **Validate models work** - run the fixed Python to ensure correct behavior
4. **Tag model files** with special naming (e.g., `*.model.py` suffix)

### Phase 3: Transpiler Analysis & Fix
1. **Compare model vs generated code** to identify exact differences
2. **Trace generation logic** in `python_visitor.rs` to find bug location
3. **Implement transpiler fix** based on model requirements
4. **Create targeted test** for the specific generation pattern

### Phase 4: Validation & Cleanup
1. **Regenerate test files** with fixed transpiler
2. **Compare regenerated vs models** - should match exactly
3. **Run comprehensive test suite** to ensure no regressions
4. **Clean up model files** once transpiler generates them correctly

## Integration with Test Framework

### Enhanced `frame_test_runner.py` Features:

```python
# New command-line options:
--create-models     # Fix broken Python files and save as models
--compare-models    # Compare generated vs model files  
--trace-diffs       # Show exact differences for transpiler fixing

# Example usage:
python3 frame_test_runner.py --create-models "broken_*.py"
python3 frame_test_runner.py --compare-models --trace-diffs
```

### Model File Management:
- **Model files:** `*.model.py` (hand-fixed working versions)
- **Generated files:** `*.py` (transpiler output)
- **Diff reports:** `*.diff.txt` (comparison details)

### Automated Model Workflow:

```python
def create_model_workflow():
    1. Identify broken generated files
    2. Apply systematic fixes using fix_runtime_errors.py
    3. Rename fixed files to *.model.py
    4. Document changes made in model_changes.log
    5. Create transpiler test cases based on models
```

## Instructions for Claude

### When I Encounter Transpiler Issues:

1. **Always run error analysis first** to understand scope and patterns
2. **For systematic issues (5+ similar errors):** Use model-driven approach
3. **For isolated issues (1-2 errors):** Fix transpiler directly
4. **Always document the approach taken** and rationale

### Model-Driven Process:
1. "I'm seeing [X pattern] errors in [Y files]. This looks systematic."
2. "Creating Python models by fixing the generated code to work correctly."
3. "Comparing models vs generated code to identify transpiler changes needed."
4. "Implementing transpiler fix based on model requirements."
5. "Regenerating and validating against models."

### Direct Fix Process:
1. "This appears to be a simple [specific bug type] in the transpiler."
2. "Fixing directly in [specific visitor/parser file]."
3. "Regenerating to validate fix."

## Benefits of This Approach

### ✅ **Advantages:**
- **Concrete targets:** Working Python shows exactly what to generate
- **Behavioral validation:** Can test logic before fixing transpiler
- **Clear comparisons:** Easy to see what transpiler should change
- **Systematic approach:** Handles complex multi-error scenarios
- **Documentation:** Models serve as examples of correct output

### ⚠️ **Cautions:**
- **Don't over-use:** Simple bugs should be fixed directly
- **Clean up models:** Remove once transpiler generates correctly
- **Validate models:** Ensure hand-fixes are actually correct
- **Track changes:** Document what was modified and why

## Example Application

### Scenario: IndentationError in 20+ files
1. **Analysis:** "Broken DEBUG comments causing systematic indentation errors"
2. **Model Creation:** Fix debug comment formatting in 5 representative files
3. **Transpiler Fix:** Modify debug comment generation in python_visitor.rs
4. **Validation:** Regenerate all files, compare with models
5. **Cleanup:** Remove model files once transpiler generates correctly

This creates a systematic, documented approach to complex transpiler debugging while maintaining engineering rigor.
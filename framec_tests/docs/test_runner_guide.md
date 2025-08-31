# Frame Test Runner Documentation

## Overview

The Frame Test Runner is the standard testing tool for the Frame language transpiler project. It provides comprehensive testing capabilities including transpilation validation, execution testing, and detailed reporting.

## Location

- **Test Runner**: `framec_tests/runner/frame_test_runner.py`
- **Test Files**: `framec_tests/python/src/`
- **Test Reports**: `framec_tests/reports/`
- **Config Files**: `framec_tests/runner/configs/`
- **Documentation**: `framec_tests/docs/`

## Features

- ✅ **Transpilation Testing**: Validates that Frame files transpile without errors
- ✅ **Execution Testing**: Runs generated Python code to verify correctness
- ✅ **Test Matrix Generation**: Creates detailed markdown reports
- ✅ **JSON Export**: Saves results in machine-readable format
- ✅ **Configurable Test Sets**: Run specific subsets of tests
- ✅ **Pattern Matching**: Test files matching specific patterns
- ✅ **Timeout Support**: Prevents hanging on infinite loops
- ✅ **Verbose Mode**: Detailed output during test runs

## Installation

The test runner requires Python 3.6+ and the Frame transpiler to be built:

```bash
# Build the Frame transpiler
cargo build

# Make test runner executable
chmod +x framec_tests/runner/frame_test_runner.py
```

## Usage

### Basic Commands

```bash
# Run all tests and generate matrix
python3 framec_tests/runner/frame_test_runner.py --all --matrix

# Run all tests with verbose output
python3 framec_tests/runner/frame_test_runner.py --all --verbose

# Run specific test pattern
python3 framec_tests/runner/frame_test_runner.py --pattern "test_import*.frm"

# Run tests from config file
python3 framec_tests/runner/frame_test_runner.py --config framec_tests/runner/configs/hsm_tests.json

# Generate both matrix and JSON output
python3 framec_tests/runner/frame_test_runner.py --all --matrix --json

# Or use the convenience wrapper from project root
./run_tests.sh
```

### Command-Line Options

| Option | Description | Default |
|--------|-------------|---------|
| `--all` | Run all test_*.frm files | - |
| `--pattern PATTERN` | Run tests matching pattern | test_*.frm |
| `--config FILE` | Run tests from JSON config | - |
| `--matrix` | Generate markdown test matrix | - |
| `--json` | Save results as JSON | - |
| `--version VERSION` | Version string for reports | v0.31 |
| `--timeout SECONDS` | Timeout for each test | 5 |
| `--verbose`, `-v` | Verbose output | False |
| `--framec PATH` | Path to framec executable | target/debug/framec |
| `--test-dir DIR` | Directory containing tests | framec_tests/python/src |
| `--output-dir DIR` | Directory for output files | docs/testing |

## Configuration Files

Test configurations are JSON files that specify which tests to run:

### Example: `configs/hsm_tests.json`
```json
{
  "name": "HSM Tests",
  "description": "Hierarchical State Machine tests",
  "tests": [
    "TestHSM.frm",
    "TestHSMBasic.frm",
    "test_parent_dispatch.frm",
    "test_forward_event.frm",
    "hierarchical/hierarchical.frm"
  ]
}
```

### Available Configurations

- `all_tests.json` - All Frame tests
- `hsm_tests.json` - Hierarchical state machine tests
- `multi_entity_tests.json` - Multi-entity/system tests
- `scope_tests.json` - Scope resolution tests

## Output Files

### Test Matrix (Markdown)

Generated at `framec_tests/reports/test_matrix_VERSION.md`:

```markdown
# Frame v0.31 Test Matrix

**Generated**: 2025-08-31 10:00  
**Total Tests**: 152  
**Current Branch**: v0.31  

## Summary Statistics

| Metric | Count | Percentage |
|--------|-------|------------|
| **Total Tests** | 152 | 100% |
| **Transpilation Success** | 150 | 98.7% |
| **Execution Success** | 143 | 94.1% |
| **Complete Success** | 143 | 94.1% |
```

### JSON Results

Generated at `framec_tests/reports/test_results_VERSION.json`:

```json
{
  "timestamp": "2025-08-31T10:00:00",
  "version": "v0.31",
  "summary": {
    "total": 152,
    "transpile_success": 150,
    "execute_success": 143,
    "complete_success": 143,
    "success_rate": "94.1%"
  },
  "results": [...]
}
```

## Test File Naming Convention

Test files must follow these conventions:
- Start with `test_`
- End with `.frm`
- Use descriptive names (e.g., `test_import_statements.frm`)
- Group related tests with common prefixes (e.g., `test_hsm_*.frm`)

## Adding New Tests

1. Create a new `.frm` file in `framec_tests/python/src/`
2. Follow the naming convention `test_FEATURE.frm`
3. Run the test individually first:
   ```bash
   python3 docs/testing/frame_test_runner.py --pattern "test_FEATURE.frm" --verbose
   ```
4. Add to appropriate config file if part of a test suite
5. Run full test suite to ensure no regressions

## Interpreting Results

### Status Indicators

- ✅ **PASS**: Both transpilation and execution succeeded
- ⚠️ **Partial**: Transpilation succeeded but execution failed
- ❌ **FAIL**: Transpilation failed

### Common Failure Types

1. **Parse Error**: Frame syntax error in source file
2. **Transpilation Timeout**: Parser hangs (usually infinite loop in parser)
3. **Runtime Error**: Generated Python has execution error
4. **Execution Timeout**: Generated code has infinite loop
5. **Import Error**: Missing Python dependencies

## Continuous Integration

The test runner can be integrated into CI pipelines:

```bash
#!/bin/bash
# ci_test.sh

# Run all tests
python3 docs/testing/frame_test_runner.py --all --matrix --json

# Check exit code
if [ $? -eq 0 ]; then
    echo "All tests passed!"
    exit 0
else
    echo "Tests failed. See test_matrix_v031.md for details."
    exit 1
fi
```

## Troubleshooting

### Tests Not Found
- Verify test files are in `framec_tests/python/src/`
- Check file permissions
- Ensure files match naming pattern

### Timeouts
- Increase timeout with `--timeout 10`
- Check for infinite loops in generated code
- Review state machine transition logic

### Import Errors
- Install required Python packages:
  ```bash
  cd framec_tests/python
  source .venv/bin/activate
  pip install -r requirements.txt
  ```

## Development

### Extending the Test Runner

The test runner is designed to be extensible. Key classes:

- `FrameTestRunner`: Main test orchestration
- `test_file()`: Individual test logic
- `generate_matrix()`: Report generation

### Adding New Report Formats

To add a new output format, extend the `FrameTestRunner` class:

```python
def generate_custom_report(self, version: str) -> str:
    """Generate custom report format"""
    # Implementation
    pass
```

## Version History

- **v1.0.0** (2025-08-31): Initial consolidated test runner
  - Merged functionality from multiple scripts
  - Added comprehensive documentation
  - Standardized configuration format

## Support

For issues or questions about the test runner:
1. Check this documentation
2. Review existing test configurations
3. Submit issues to the Frame project repository
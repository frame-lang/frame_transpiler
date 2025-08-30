# Frame Python Tests

This directory contains the Python test suite for the Frame transpiler v0.30.

## Prerequisites

- Python 3.8 or higher
- Frame transpiler built (`cargo build` in project root)

## Quick Start

### 1. First-Time Setup

```bash
# Navigate to Python tests directory
cd framec_tests/python

# Create and activate virtual environment
python3 -m venv .venv
source .venv/bin/activate  # On Windows: .venv\Scripts\activate

# Install dependencies (if requirements.txt exists)
pip install -r requirements.txt
```

### 2. Running Tests

```bash
# Make sure virtual environment is activated
source .venv/bin/activate

# Run all tests
python3 scripts/frame_test_runner.py

# Run specific test patterns
python3 scripts/frame_test_runner.py "test_enum*.frm"
python3 scripts/frame_test_runner.py "test_functions*.frm"

# Use configuration files
python3 scripts/frame_test_runner.py -c configs/all_tests.json

# Verbose output for debugging
python3 scripts/frame_test_runner.py -v

# Only transpile without running (useful for debugging)
python3 scripts/frame_test_runner.py -t "test_*.frm"

# Force re-transpilation
python3 scripts/frame_test_runner.py -f "test_*.frm"
```

## Directory Structure

```
framec_tests/python/
├── README.md                # This file
├── requirements.txt         # Python dependencies (when needed)
├── .venv/                  # Virtual environment (git-ignored)
├── src/                    # Frame test files (.frm)
│   ├── test_*.frm         # Individual test files
│   └── *.py               # Generated Python files (git-ignored)
├── scripts/
│   └── frame_test_runner.py # Main test runner
└── configs/                # Test configuration files
    ├── all_tests.json
    ├── enum_tests.json
    └── multi_entity_tests.json
```

## Adding New Tests

1. Create a `.frm` file in the `src/` directory
2. Follow naming convention: `test_<feature_name>.frm`
3. Include a `main()` function with verifiable output
4. Run the test: `python3 scripts/frame_test_runner.py test_<feature_name>.frm`

### Test File Template

```frame
fn main() {
    // Test code with verifiable output
    print("Expected output")
}

system TestSystem {
    // System definition
}
```

## Adding Dependencies

If your test requires additional Python packages:

1. Create/update `requirements.txt` in this directory
2. Add the package (e.g., `jsonpickle>=2.0.0`)
3. Run `pip install -r requirements.txt` (with venv activated)
4. Use backticks in Frame code for Python-specific imports:
   ```frame
   `import jsonpickle`
   ```

## Troubleshooting

### Virtual Environment Not Working
```bash
# Remove and recreate
rm -rf .venv
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt  # if it exists
```

### Import Errors
- Ensure virtual environment is activated: `source .venv/bin/activate`
- Check that dependencies are installed: `pip list`
- Install missing packages: `pip install <package_name>`

### Test Failures
- Use verbose mode: `python3 scripts/frame_test_runner.py -v test_name.frm`
- Check generated Python file in `src/` directory
- Verify Frame transpiler is built: `cargo build` in project root

### Generated Files
- Python files (`.py`) are generated in the `src/` directory
- These files are git-ignored and regenerated as needed
- Use `-f` flag to force regeneration

## Test Categories

- **Basic Syntax**: `test_simple_*.frm`, `test_minimal_*.frm`
- **Enums**: `test_enum*.frm`
- **Functions**: `test_functions_*.frm`
- **Multi-Entity**: `test_multi_*.frm`, `test_v030_*.frm`
- **Hierarchical State Machines**: `test_hsm_*.frm`, `test_parent_*.frm`
- **Operations**: `test_operations_*.frm`
- **Scope**: `test_scope_*.frm`, `test_legb_*.frm`

## Contributing

When adding or modifying tests:

1. Ensure tests have verifiable output (use `print()` statements)
2. Test both positive and negative cases
3. Document any special requirements in comments
4. Run the full test suite before committing
5. Update relevant documentation if needed

## CI/CD Integration

For continuous integration, use these commands:

```bash
# Setup
cd framec_tests/python
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt  # if it exists

# Run tests
python3 scripts/frame_test_runner.py -c configs/all_tests.json
```

## Current Test Success Rate

As of latest run: **97.3%** (142/146 tests passing)

Run `python3 scripts/frame_test_runner.py` to see current statistics.
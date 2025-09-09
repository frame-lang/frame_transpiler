# Frame Test Environment Setup

## Overview

Some Frame tests require external Python packages (e.g., NumPy for matrix multiplication operator `@`). This guide explains how to set up a proper test environment.

## Quick Start

```bash
# Navigate to the test directory
cd framec_tests

# Create a virtual environment
python3 -m venv venv

# Activate the virtual environment
source venv/bin/activate  # On macOS/Linux
# or
venv\Scripts\activate     # On Windows

# Install test dependencies
pip install -r requirements.txt

# Run tests
python3 runner/frame_test_runner.py --all
```

## Test Dependencies

### Core Dependencies (Always Required)
- Python 3.8 or higher
- No additional packages needed for basic Frame features

### Optional Dependencies

#### NumPy (for Matrix Multiplication)
Required for tests using the `@` operator with actual matrix operations:
- `test_matmul_with_numpy.frm`
- Any test performing actual matrix multiplication

```bash
pip install numpy
```

#### Other Future Dependencies
As Frame evolves, additional packages may be needed for specific features.

## Virtual Environment Management

### Creating a Virtual Environment

```bash
# Standard approach
python3 -m venv venv

# With specific Python version
python3.11 -m venv venv
```

### Activating the Environment

```bash
# macOS/Linux
source venv/bin/activate

# Windows
venv\Scripts\activate

# Verify activation
which python  # Should show path to venv/bin/python
```

### Deactivating

```bash
deactivate
```

## Test Categories by Requirements

### No Dependencies Required
Most Frame tests run without any external dependencies:
- Basic syntax tests
- State machine tests  
- Module system tests
- Control flow tests
- Standard library function tests

### NumPy Required
Tests that use actual matrix operations:
- `test_matmul_with_numpy.frm` - Full matrix multiplication testing
- Future scientific computing tests

### Transpilation-Only Tests
These tests validate syntax generation without execution:
- `test_matmul_syntax_only.frm` - Validates @ operator transpilation
- `test_matmul_transpile.frm` - Checks code generation

## Running Tests with Dependencies

### Check Available Dependencies

```python
# Check if NumPy is available
python3 -c "import numpy; print(f'NumPy {numpy.__version__} available')"
```

### Running Specific Test Suites

```bash
# Run all tests (some may skip if dependencies missing)
python3 runner/frame_test_runner.py --all

# Run only NumPy tests
python3 runner/frame_test_runner.py --config configs/numpy_tests.json

# Run tests that don't require dependencies
python3 runner/frame_test_runner.py --config configs/basic_tests.json
```

## Handling Missing Dependencies

Frame tests should handle missing dependencies gracefully:

```frame
# Example test with optional NumPy dependency
import numpy as np

fn test_feature() {
    try {
        # NumPy-dependent code
        var matrix = np.array([[1, 2], [3, 4]])
        var result = matrix @ matrix
        print("Matrix multiplication successful")
    } except {
        print("NumPy not available - skipping matrix tests")
        print("However, @ operator syntax transpiles correctly")
    }
}
```

## CI/CD Considerations

For continuous integration:

1. **Basic Test Suite**: Run without dependencies to validate core functionality
2. **Full Test Suite**: Include all dependencies for comprehensive testing
3. **Dependency Matrix**: Test against multiple Python/NumPy versions

Example GitHub Actions workflow:

```yaml
- name: Set up Python
  uses: actions/setup-python@v4
  with:
    python-version: '3.11'

- name: Install dependencies
  run: |
    python -m pip install --upgrade pip
    pip install -r framec_tests/requirements.txt

- name: Run tests
  run: |
    cd framec_tests
    python3 runner/frame_test_runner.py --all --json
```

## Troubleshooting

### Import Errors
If you see `ModuleNotFoundError: No module named 'numpy'`:
1. Ensure virtual environment is activated
2. Install NumPy: `pip install numpy`
3. Verify installation: `pip list | grep numpy`

### Version Conflicts
If tests fail due to version issues:
1. Check required versions in `requirements.txt`
2. Update packages: `pip install --upgrade numpy`
3. Or install specific version: `pip install numpy==1.24.0`

### Virtual Environment Not Working
1. Ensure you're in the correct directory
2. Check Python version: `python3 --version`
3. Recreate venv if needed: `rm -rf venv && python3 -m venv venv`

## Best Practices

1. **Always use virtual environments** for testing to avoid system package conflicts
2. **Document dependencies** in test files that require them
3. **Handle missing dependencies gracefully** with try-except blocks
4. **Keep requirements.txt updated** with exact versions for reproducibility
5. **Test both with and without** optional dependencies to ensure graceful degradation

## Future Enhancements

Planned improvements to test infrastructure:
- Automatic dependency detection from test files
- Skip decorators for tests with missing dependencies
- Dependency installation prompts in test runner
- Docker containers with pre-configured environments
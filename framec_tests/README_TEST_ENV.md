# Frame Test Environment Setup

## Quick Setup

For the fastest setup, use our automated script:

```bash
cd framec_tests
./setup_test_env.sh
```

This will:
1. Create a Python virtual environment
2. Install all test dependencies including NumPy
3. Verify the installation

## Manual Setup

If you prefer manual setup:

```bash
cd framec_tests

# Create virtual environment
python3 -m venv venv

# Activate it
source venv/bin/activate  # macOS/Linux
# or
venv\Scripts\activate     # Windows

# Install dependencies
pip install -r requirements.txt
```

## Running Tests

Once your environment is set up:

```bash
# Make sure virtual environment is activated
source venv/bin/activate

# Run all tests
python3 runner/frame_test_runner.py --all

# Run specific test categories
python3 runner/frame_test_runner.py --config configs/numpy_tests.json
```

## Test Categories

### Tests that work without dependencies
Most Frame tests (>98%) work without any external packages:
- Basic syntax tests
- State machine tests
- Module system tests
- Standard library tests

### Tests requiring NumPy
Only a few tests need NumPy for matrix multiplication (`@` operator):
- `test_matmul_with_numpy.frm` - Full matrix multiplication
- Future scientific computing tests

### Transpilation-only tests
These validate syntax without execution:
- `test_matmul_syntax_only.frm` - Validates @ operator generation
- `test_matmul_transpile.frm` - Checks transpilation

## Dependency Notes

- **NumPy**: Required only for actual matrix multiplication execution. The `@` operator will transpile correctly even without NumPy, but won't execute on regular Python types.
- **Other packages**: Listed in requirements.txt for future async and testing features

## Troubleshooting

### NumPy installation fails
```bash
# Try upgrading pip first
pip install --upgrade pip

# Then install NumPy
pip install numpy

# Or install specific version
pip install numpy==1.24.0
```

### Tests fail with import errors
Make sure your virtual environment is activated:
```bash
which python  # Should show path to venv/bin/python
```

## System vs Virtual Environment

**We strongly recommend using a virtual environment** to:
- Avoid conflicts with system packages
- Ensure reproducible test results
- Allow easy cleanup
- Test with specific package versions

The Frame transpiler itself doesn't require NumPy - only certain tests that validate the matrix multiplication operator do.
# Frame Transpiler Test Setup

## Quick Start

### First Time Setup
```bash
# Clone the repository
git clone https://github.com/frame-lang/frame_transpiler.git
cd frame_transpiler

# Build the Rust transpiler
cargo build --release

# Setup Python test environment
chmod +x setup_tests.sh
./setup_tests.sh
```

### Running Tests
```bash
# Activate Python environment
source venv/bin/activate

# Run all tests
cd framec_tests
python3 runner/frame_test_runner.py --all

# Run specific test
python3 runner/frame_test_runner.py test_async_*.frm
```

## Development Workflow

### For Rust Development
```bash
# Make changes to transpiler
vim src/frame_c/visitors/python_visitor.rs

# Build and test
cargo build --release
source venv/bin/activate
cd framec_tests
python3 runner/frame_test_runner.py --all
```

### For Test Development
```bash
# Activate environment
source venv/bin/activate

# Create new test
cd framec_tests/python/src
vim test_new_feature.frm

# Transpile and run
../../../target/release/framec -l python_3 test_new_feature.frm > test_new_feature.py
python3 test_new_feature.py
```

## VSCode Setup

1. Open the project in VSCode
2. Install the Python extension
3. Press `Cmd+Shift+P` → "Python: Select Interpreter"
4. Choose `./venv/bin/python`
5. Open integrated terminal - it should auto-activate venv

## CI/CD Integration

Tests run automatically on GitHub Actions for each PR. See `.github/workflows/test.yml`

## Troubleshooting

### Missing module errors
```bash
# Ensure venv is activated
source venv/bin/activate
# Reinstall dependencies
pip install -r framec_tests/requirements.txt
```

### Permission denied on setup_tests.sh
```bash
chmod +x setup_tests.sh
```

### Tests fail with import errors
Make sure you're using the virtual environment Python, not system Python:
```bash
which python3  # Should show: /path/to/frame_transpiler/venv/bin/python3
```

## Dependencies

- **Rust**: For building the transpiler
- **Python 3.9+**: For running generated code
- **pip**: For installing Python packages

### Python packages (auto-installed by setup):
- `aiohttp`: For async/await tests with HTTP
- `asyncio`: For async runtime
- `pytest`: For test framework
- `jsonpickle`: For serialization tests

## Project Structure
```
frame_transpiler/
├── Cargo.toml                 # Rust project configuration
├── src/                        # Rust transpiler source
├── target/                     # Rust build output
│   └── release/
│       └── framec             # The transpiler executable
├── venv/                       # Python virtual environment (git-ignored)
├── framec_tests/
│   ├── requirements.txt       # Python dependencies
│   ├── runner/
│   │   └── frame_test_runner.py  # Test runner
│   └── python/
│       └── src/
│           ├── *.frm          # Frame source files
│           └── *.py           # Generated Python files
└── setup_tests.sh             # This setup script
```
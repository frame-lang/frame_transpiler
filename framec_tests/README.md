# Frame Tests

This directory contains all testing infrastructure for the Frame language transpiler.

## Directory Structure

```
framec_tests/
├── docs/                 # Testing documentation
│   └── test_runner_guide.md
├── runner/              # Test runner and tools
│   ├── frame_test_runner.py
│   ├── configs/         # Test configuration files
│   │   ├── all_tests.json
│   │   ├── hsm_tests.json
│   │   ├── multi_entity_tests.json
│   │   └── scope_tests.json
│   └── validate_*.sh    # Legacy validation scripts
├── reports/             # Generated test reports
│   ├── test_matrix_v031.md
│   └── test_results_v031.json
└── python/              # Python-specific tests
    ├── src/             # Frame test source files (.frm)
    │   └── test_*.frm   # Individual test files
    ├── generated/       # Generated Python files (deprecated)
    └── .venv/           # Python virtual environment
```

## Quick Start

From the project root directory:

```bash
# Run all tests with default settings
./run_tests.sh

# Run specific test pattern
./run_tests.sh --pattern "test_import*.frm"

# Run with verbose output
./run_tests.sh --all --verbose

# Generate both matrix and JSON reports
./run_tests.sh --all --matrix --json
```

## Test Runner

The main test runner is located at `runner/frame_test_runner.py`. It provides:
- Transpilation validation
- Execution testing
- Report generation
- Configurable test sets

See `docs/test_runner_guide.md` for detailed documentation.

## Test Reports

Generated reports are saved to the `reports/` directory:
- `test_matrix_VERSION.md` - Human-readable markdown report
- `test_results_VERSION.json` - Machine-readable JSON results

## Adding New Tests

1. Create a new `.frm` file in `python/src/`
2. Follow naming convention: `test_FEATURE.frm`
3. Test individually: `./run_tests.sh --pattern "test_FEATURE.frm"`
4. Add to appropriate config if part of a suite

## Test Configurations

Pre-defined test suites in `runner/configs/`:
- `all_tests.json` - Complete test suite
- `hsm_tests.json` - Hierarchical state machine tests
- `multi_entity_tests.json` - Multi-entity/system tests
- `scope_tests.json` - Scope resolution tests

## Current Test Status (v0.31)

- **Total Tests**: 152
- **Success Rate**: 94.1% (143/152)
- **Transpilation Success**: 98.7% (150/152)

See latest report in `reports/test_matrix_v031.md` for details.
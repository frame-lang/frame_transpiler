# Frame Tests - Unified Testing Structure

This directory contains the unified testing infrastructure for the Frame language transpiler, supporting multiple target languages with a common test suite.

## Directory Structure

```
framec_tests/
├── common/                    # Shared tests for all languages
│   └── tests/
│       ├── core/              # Core Frame features (31 tests)
│       ├── control_flow/      # Control flow tests (48 tests)
│       ├── data_types/        # Data type tests (66 tests)
│       ├── operators/         # Operator tests (16 tests)
│       ├── scoping/           # Scoping tests (55 tests)
│       ├── systems/           # System tests (197 tests)
│       ├── regression/        # Bug regression tests (6 tests)
│       └── negative/          # Expected failure tests (13 tests)
│
├── language_specific/         # Language-specific tests
│   ├── python/               # Python-specific features (29 tests)
│   └── typescript/           # TypeScript-specific features
│
├── generated/                # Generated code (gitignored)
│   ├── python/              # Generated .py files
│   └── typescript/          # Generated .ts files
│
├── runner/                  # Test infrastructure
│   └── frame_test_runner.py # Unified test runner
│
└── reports/                 # Test reports and results
```

## Test Categories

### Common Tests (432 tests)
These tests validate core Frame functionality that should work identically across all target languages:

- **core** (31 tests): State machines, events, transitions, handlers, operations
- **control_flow** (48 tests): if/else, loops, return, continue, break
- **data_types** (66 tests): Lists, dicts, sets, strings, numbers, comprehensions
- **operators** (16 tests): Arithmetic, logical, comparison, bitwise operators
- **scoping** (55 tests): Variable scoping, module scope, self references
- **systems** (197 tests): System definitions, parameters, multi-system tests
- **regression** (6 tests): Bug fixes (bug29, bug30, bug31, bug35, etc.)
- **negative** (13 tests): Tests that should fail (syntax errors, validation)

### Language-Specific Tests (29 tests)
Tests for features that are specific to certain target languages:

- **Python** (29 tests):
  - async/await syntax
  - Decorators (@dataclass, @property)
  - Import statements
  - With statements
  - C-style comments handling
  - Walrus operator

## Quick Start

### Run All Tests for All Languages
```bash
cd /Users/marktruluck/projects/frame_transpiler
python3 framec_tests/runner/frame_test_runner.py
```

### Run Tests for Specific Language
```bash
# Python only
python3 framec_tests/runner/frame_test_runner.py --languages python

# TypeScript only  
python3 framec_tests/runner/frame_test_runner.py --languages typescript

# Both
python3 framec_tests/runner/frame_test_runner.py --languages python typescript
```

### Run Specific Test Categories
```bash
# Core tests only
python3 framec_tests/runner/frame_test_runner.py --categories core

# Multiple categories
python3 framec_tests/runner/frame_test_runner.py --categories core control_flow data_types

# Language-specific tests
python3 framec_tests/runner/frame_test_runner.py --categories language_specific_python
```

### Transpile Only (No Execution)
```bash
python3 framec_tests/runner/frame_test_runner.py --transpile-only
```

### Verbose Output
```bash
python3 framec_tests/runner/frame_test_runner.py --verbose
```

### Generate Report
```bash
python3 framec_tests/runner/frame_test_runner.py --output report.json
```

## Test Runner Options

```
--languages, -l      Languages to test (python, typescript, rust, golang, javascript)
--categories, -c     Test categories to run (default: all)
--framec            Path to framec executable (default: ./target/release/framec)
--verbose, -v       Verbose output showing each test result
--transpile-only    Only transpile, do not execute generated code
--output, -o        Output JSON report to file
--timeout           Timeout for each test in seconds (default: 10)
```

## Current Test Status

### Python (v0.82.1)
- Total: 461 tests
- Common: 432 tests
- Language-specific: 29 tests
- Success rate: 90% transpilation (417/461)
- Known issues: 6 core tests with unreachable return statements

### TypeScript (v0.82.1)
- Total: 432 common tests available
- Language-specific: 0 tests (new target)
- Success rate: 80.6% transpilation (350/432)
- Known issues: Same 6 core tests as Python

### Recent Fixes
- **Bug #50 Fixed (2025-10-13)**: Language-specific tests now properly filtered by category

## Adding New Tests

### Common Test
Create a `.frm` file in the appropriate category under `common/tests/`:

```bash
# Example: Add a new state machine test
echo 'system NewTest { ... }' > common/tests/core/test_new_feature.frm
```

### Language-Specific Test
Create a `.frm` file under `language_specific/<language>/`:

```bash
# Example: Add Python-specific async test
echo '...' > language_specific/python/test_async_context.frm
```

## Migration from Old Structure

The migration has been completed. The old structure is backed up in `framec_tests_old/`.

## Benefits of New Structure

1. **Single Source of Truth**: One `.frm` file tests all languages
2. **Better Organization**: Tests categorized by feature, not scattered
3. **Language Parity**: Easy to see which languages support which features
4. **Simplified Maintenance**: Fix one test, all languages benefit
5. **Clear Separation**: Language-specific features explicitly segregated
6. **Scalable**: Easy to add new languages or test categories

## Future Enhancements

- [ ] Parallel test execution
- [ ] Test coverage metrics
- [ ] Performance benchmarking
- [ ] Automatic categorization of new tests
- [ ] Visual test matrix dashboard
- [ ] Integration with CI/CD
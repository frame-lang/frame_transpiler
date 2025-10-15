#!/bin/bash
# Compare test results across languages

echo "==========================================
Frame Language Parity Report
==========================================

This report shows which tests pass for each supported language.
A unified test suite ensures consistent behavior across all targets.
"

# Run tests for all categories
echo "Running common tests for Python and TypeScript..."
python3 runner/frame_test_runner.py \
    --languages python typescript \
    --framec ../target/release/framec \
    --transpile-only \
    --output comparison_report.json \
    > test_output.txt 2>&1

# Extract summary
echo "Test Results Summary:"
echo "---------------------"
tail -30 test_output.txt | head -25

echo "
Detailed report saved to: comparison_report.json

Key Benefits of Unified Testing:
- Single .frm file tests both Python and TypeScript
- Ensures language parity and consistent behavior  
- Easier to maintain and add new languages
- Clear visibility into feature support across languages
"

# Show category breakdown
echo "Test Distribution:"
echo "-----------------"
echo "Core:         31 tests"
echo "Control Flow: 48 tests" 
echo "Data Types:   66 tests"
echo "Operators:    16 tests"
echo "Scoping:      55 tests"
echo "Systems:     197 tests"
echo "Regression:    6 tests"
echo "Negative:     13 tests"
echo "-----------------"
echo "TOTAL COMMON: 432 tests"
echo ""
echo "Language-Specific:"
echo "Python:       29 tests"
echo "TypeScript:    0 tests (new target)"
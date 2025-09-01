#!/bin/bash

# Frame Transpiler Test Validation Script
# Tests all .frm files in framec_tests/python/src

echo "Frame Transpiler Test Validation"
echo "================================"
echo ""

# Build the transpiler first
echo "Building transpiler..."
cargo build --bin framec --release 2>/dev/null || cargo build --bin framec
if [ $? -ne 0 ]; then
    echo "ERROR: Failed to build transpiler"
    exit 1
fi

# Find the framec binary
if [ -f "./target/release/framec" ]; then
    FRAMEC="./target/release/framec"
else
    FRAMEC="./target/debug/framec"
fi

echo "Using transpiler: $FRAMEC"
echo ""

# Initialize counters
TOTAL=0
PASSED=0
FAILED=0
TRANSPILE_ERRORS=0
RUNTIME_ERRORS=0

# Create temporary directory for results
RESULTS_DIR="test_results_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$RESULTS_DIR"

echo "Test Results Directory: $RESULTS_DIR"
echo ""
echo "Running tests..."
echo "----------------"

# Process each .frm file
for frm_file in framec_tests/python/src/**/*.frm framec_tests/python/src/*.frm; do
    if [ ! -f "$frm_file" ]; then
        continue
    fi
    
    TOTAL=$((TOTAL + 1))
    
    # Get base name and directory
    base_name=$(basename "$frm_file" .frm)
    dir_name=$(dirname "$frm_file")
    py_file="$dir_name/$base_name.py"
    
    # Short name for display
    short_name="${frm_file#framec_tests/python/src/}"
    
    printf "%-50s" "$short_name"
    
    # Transpile
    $FRAMEC -l python_3 "$frm_file" > "$py_file" 2>"$RESULTS_DIR/${base_name}_transpile.err"
    
    if [ $? -ne 0 ]; then
        echo " [TRANSPILE ERROR]"
        FAILED=$((FAILED + 1))
        TRANSPILE_ERRORS=$((TRANSPILE_ERRORS + 1))
        echo "  Error details in: $RESULTS_DIR/${base_name}_transpile.err"
        continue
    fi
    
    # Run Python
    python3 "$py_file" > "$RESULTS_DIR/${base_name}_output.txt" 2>"$RESULTS_DIR/${base_name}_runtime.err"
    
    if [ $? -ne 0 ]; then
        echo " [RUNTIME ERROR]"
        FAILED=$((FAILED + 1))
        RUNTIME_ERRORS=$((RUNTIME_ERRORS + 1))
        echo "  Error details in: $RESULTS_DIR/${base_name}_runtime.err"
    else
        echo " [PASSED]"
        PASSED=$((PASSED + 1))
    fi
done

echo ""
echo "================================"
echo "Test Summary"
echo "================================"
echo "Total tests:        $TOTAL"
echo "Passed:            $PASSED"
echo "Failed:            $FAILED"
echo "  - Transpile errors: $TRANSPILE_ERRORS"
echo "  - Runtime errors:   $RUNTIME_ERRORS"
echo ""

# Calculate pass rate
if [ $TOTAL -gt 0 ]; then
    PASS_RATE=$(echo "scale=1; $PASSED * 100 / $TOTAL" | bc)
    echo "Pass rate: ${PASS_RATE}%"
else
    echo "No tests found!"
fi

echo ""
echo "Results saved in: $RESULTS_DIR"

# Exit with error if any tests failed
if [ $FAILED -gt 0 ]; then
    exit 1
fi
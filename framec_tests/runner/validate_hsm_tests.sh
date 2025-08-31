#!/bin/bash

# HSM Test Validation Script for Frame v0.30
# Tests all Hierarchical State Machine related files

TRANSPILER="/Users/marktruluck/projects/frame_transpiler/target/debug/framec"
TEST_DIR="/Users/marktruluck/projects/frame_transpiler/framec_tests/python/src"
OUTPUT_DIR="/tmp/hsm_tests"

# Create output directory
mkdir -p "$OUTPUT_DIR"

# List of HSM test files
HSM_FILES=(
    "TestHSM.frm"
    "TestHSMBasic.frm"
    "TestContinue.frm"
    "test_parent_dispatch.frm"
    "test_parent_dispatch_complete.frm"
    "test_parent_transition_detection.frm"
    "test_forward_event.frm"
    "test_history.frm"
    "hierarchical/hierarchical.frm"
    "history101/history101.frm"
    "history102/history102.frm"
    "history103/history103.frm"
    "history104/history104.frm"
    "history105/history105.frm"
)

echo "=== Frame v0.30 HSM Test Validation ==="
echo "Testing ${#HSM_FILES[@]} HSM-related files"
echo ""

PASSED=0
FAILED=0
ERRORS=()

for file in "${HSM_FILES[@]}"; do
    echo -n "Testing $file... "
    
    # Full path to the Frame file
    FRAME_FILE="$TEST_DIR/$file"
    
    # Output Python file
    OUTPUT_FILE="$OUTPUT_DIR/$(basename $file .frm).py"
    
    # Check if Frame file exists
    if [ ! -f "$FRAME_FILE" ]; then
        echo "NOT FOUND"
        FAILED=$((FAILED + 1))
        ERRORS+=("$file: File not found")
        continue
    fi
    
    # Transpile to Python
    if $TRANSPILER -l python_3 "$FRAME_FILE" > "$OUTPUT_FILE" 2>/dev/null; then
        # Check if output has content
        if [ -s "$OUTPUT_FILE" ]; then
            # Try to compile the Python code
            if python3 -m py_compile "$OUTPUT_FILE" 2>/dev/null; then
                echo "✅ PASSED"
                PASSED=$((PASSED + 1))
            else
                echo "❌ PYTHON SYNTAX ERROR"
                FAILED=$((FAILED + 1))
                ERRORS+=("$file: Generated Python has syntax errors")
            fi
        else
            echo "❌ EMPTY OUTPUT"
            FAILED=$((FAILED + 1))
            ERRORS+=("$file: Transpiler produced empty output")
        fi
    else
        echo "❌ TRANSPILER ERROR"
        FAILED=$((FAILED + 1))
        ERRORS+=("$file: Transpiler failed")
    fi
done

echo ""
echo "=== Summary ==="
echo "Passed: $PASSED/${#HSM_FILES[@]}"
echo "Failed: $FAILED/${#HSM_FILES[@]}"

if [ ${#ERRORS[@]} -gt 0 ]; then
    echo ""
    echo "=== Errors ==="
    for error in "${ERRORS[@]}"; do
        echo "  - $error"
    done
fi

echo ""
echo "Generated files saved to: $OUTPUT_DIR"
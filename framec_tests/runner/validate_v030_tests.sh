#!/bin/bash

# Validate v0.30 multi-entity test files

echo "=== Frame v0.30 Multi-Entity Test Validation ==="
echo ""

# Test files to validate
test_files=(
    "test_v030_functions_only.frm"
    "test_v030_multi_system_basic.frm"
    "test_v030_mixed_entities.frm"
    "test_v030_system_with_functions.frm"
    "test_v030_three_systems.frm"
    "test_single_system_only.frm"
    "test_minimal_two_systems.frm"
    "test_multiple_systems_valid.frm"
    "test_two_systems_no_function.frm"
    "test_two_systems_print.frm"
)

FRAMEC="../../../target/debug/framec"
SRC_DIR="."

total=0
passed=0
failed=0

for test_file in "${test_files[@]}"; do
    if [ ! -f "$SRC_DIR/$test_file" ]; then
        echo "❌ $test_file - FILE NOT FOUND"
        continue
    fi
    
    echo "Testing: $test_file"
    total=$((total + 1))
    
    # Try to generate Python code
    output_file="${test_file%.frm}.py"
    
    if timeout 5 $FRAMEC -l python_3 "$SRC_DIR/$test_file" > "$SRC_DIR/$output_file" 2>&1; then
        echo "  ✅ Generation successful"
        
        # Try to run the generated Python
        if timeout 5 python3 "$SRC_DIR/$output_file" 2>&1; then
            echo "  ✅ Execution successful"
            passed=$((passed + 1))
        else
            echo "  ❌ Execution failed"
            failed=$((failed + 1))
        fi
    else
        echo "  ❌ Generation failed or timed out"
        failed=$((failed + 1))
        
        # Check if it's hanging
        if ! timeout 2 $FRAMEC -l python_3 "$SRC_DIR/$test_file" 2>&1 > /dev/null; then
            echo "  ⚠️  Parser appears to be hanging on this file"
        fi
    fi
    echo ""
done

echo "=== Summary ==="
echo "Total tests: $total"
echo "Passed: $passed"
echo "Failed: $failed"
echo "Success rate: $(( passed * 100 / total ))%"
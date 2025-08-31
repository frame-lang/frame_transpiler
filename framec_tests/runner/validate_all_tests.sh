#!/bin/bash

# Comprehensive Frame Test Validation Script
# Generates Python from every .frm file and runs it to validate functionality

echo "=== Frame Test Validation Suite ==="
echo "Processing all .frm files in framec_tests/python/src/"
echo ""

# Counters
total=0
passed=0
failed=0
generation_failed=0

# Change to framec root directory
cd /Users/marktruluck/projects/frame_transpiler

# Results file
results_file="framec_tests/python/src/validation_results.txt"
echo "Frame Test Validation Results - $(date)" > "$results_file"
echo "========================================" >> "$results_file"
echo "" >> "$results_file"

# Find all .frm files (excluding .disabled)
find framec_tests/python/src -name "*.frm" -not -name "*.disabled" | sort | while read frm_file; do
    total=$((total + 1))
    
    # Get relative path and base name
    relative_path=${frm_file#framec_tests/python/src/}
    base_name=$(basename "$frm_file" .frm)
    dir_name=$(dirname "$frm_file")
    py_file="$dir_name/$base_name.py"
    
    echo "=== Testing $relative_path [$total] ==="
    
    # Step 1: Generate Python code
    if ./target/debug/framec -l python_3 "$frm_file" > "$py_file" 2>&1; then
        echo "  ✓ Generation successful"
        
        # Step 2: Run the generated Python code
        cd "$(dirname "$py_file")"
        if timeout 10s python3 "$(basename "$py_file")" > /dev/null 2>&1; then
            echo "  ✓ Runtime successful"
            echo "PASS: $relative_path" >> "/Users/marktruluck/projects/frame_transpiler/$results_file"
            passed=$((passed + 1))
        else
            echo "  ✗ Runtime failed"
            echo "RUNTIME_FAIL: $relative_path" >> "/Users/marktruluck/projects/frame_transpiler/$results_file"
            failed=$((failed + 1))
        fi
        cd /Users/marktruluck/projects/frame_transpiler
    else
        echo "  ✗ Generation failed"
        echo "GENERATION_FAIL: $relative_path" >> "$results_file"
        generation_failed=$((generation_failed + 1))
    fi
    
    echo ""
done

# Final summary
echo "=== VALIDATION SUMMARY ===" 
echo "Total files: $total"
echo "Passed: $passed"
echo "Runtime failed: $failed" 
echo "Generation failed: $generation_failed"
echo ""
echo "Results written to: $results_file"
#!/bin/bash

# Comprehensive Frame Testing Script
# Integrates transpilation testing with existing pytest framework

FRAMEC="../../../target/debug/framec"
SRC_DIR="../src"
GENERATED_DIR="../generated"
RESULTS_FILE="test_results_comprehensive.txt"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Frame Comprehensive Testing Suite${NC}"
echo "=================================="
echo "Tests stored in: $SRC_DIR"
echo "Generated code: $GENERATED_DIR"
echo ""

# Clear results file
echo "Frame Comprehensive Test Results - $(date)" > "$RESULTS_FILE"
echo "=============================================" >> "$RESULTS_FILE"

# Counters
TOTAL=0
TRANSPILED=0
EXECUTED=0
VALIDATION_PASSED=0
VALIDATION_FAILED=0
PYTEST_PASSED=0

# Step 1: Regenerate all transpiled files
echo -e "${BLUE}Step 1: Transpiling Frame files...${NC}"
echo ""

mkdir -p "$GENERATED_DIR"

for frm_file in $(find "$SRC_DIR" -name "*.frm" -type f | sort); do
    TOTAL=$((TOTAL + 1))
    
    # Get relative path and create output path
    rel_path="${frm_file#$SRC_DIR/}"
    base_name=$(basename "$frm_file" .frm)
    dir_name=$(dirname "$rel_path")
    
    # Create subdirectory in generated if needed
    if [ "$dir_name" != "." ]; then
        mkdir -p "$GENERATED_DIR/$dir_name"
        py_file="$GENERATED_DIR/$dir_name/$base_name.py"
    else
        py_file="$GENERATED_DIR/$base_name.py"
    fi
    
    # Transpile the file
    echo -n "[$TOTAL] $rel_path... "
    if $FRAMEC -l python_3 "$frm_file" > "$py_file" 2>/dev/null; then
        TRANSPILED=$((TRANSPILED + 1))
        echo -e "${GREEN}TRANSPILED${NC}"
        echo "[$TOTAL] $rel_path - TRANSPILED" >> "$RESULTS_FILE"
        
        # Test execution and validation
        if grep -q "EXPECTED_OUTPUT:" "$frm_file"; then
            expected=$(grep "EXPECTED_OUTPUT:" "$frm_file" | head -1 | sed 's/.*EXPECTED_OUTPUT: *//')
            
            output=$(python3 "$py_file" 2>&1)
            exit_code=$?
            
            if [ $exit_code -eq 0 ]; then
                EXECUTED=$((EXECUTED + 1))
                
                if [ "$output" = "$expected" ]; then
                    VALIDATION_PASSED=$((VALIDATION_PASSED + 1))
                    echo "    ${GREEN}✓ VALIDATION PASS${NC}: '$output'"
                    echo "    VALIDATION PASS: '$output'" >> "$RESULTS_FILE"
                else
                    VALIDATION_FAILED=$((VALIDATION_FAILED + 1))
                    echo "    ${YELLOW}✗ VALIDATION MISMATCH${NC}"
                    echo "      Expected: '$expected'"
                    echo "      Got:      '$output'"
                    echo "    VALIDATION MISMATCH: expected='$expected', got='$output'" >> "$RESULTS_FILE"
                fi
            else
                VALIDATION_FAILED=$((VALIDATION_FAILED + 1))
                echo "    ${RED}✗ RUNTIME ERROR${NC}: $output"
                echo "    RUNTIME ERROR: $output" >> "$RESULTS_FILE"
            fi
        else
            # Try to run without validation
            output=$(python3 "$py_file" 2>&1)
            exit_code=$?
            
            if [ $exit_code -eq 0 ]; then
                EXECUTED=$((EXECUTED + 1))
                echo "    ${GREEN}✓ EXECUTED${NC}"
            else
                echo "    ${RED}✗ RUNTIME ERROR${NC}"
            fi
        fi
    else
        echo -e "${RED}FAILED${NC}"
        echo "[$TOTAL] $rel_path - TRANSPILE FAILED" >> "$RESULTS_FILE"
    fi
done

# Step 2: Run existing pytest framework  
echo ""
echo -e "${BLUE}Step 2: Running pytest framework...${NC}"
echo ""

cd ../
if python -m pytest src/tests/ -v --tb=short; then
    PYTEST_PASSED=1
    echo "PYTEST: PASSED" >> "scripts/$RESULTS_FILE"
else
    echo "PYTEST: FAILED" >> "scripts/$RESULTS_FILE"
fi

# Step 3: Summary
cd scripts/
echo ""
echo -e "${BLUE}=========================================${NC}"
echo -e "${BLUE}Comprehensive Test Summary${NC}"
echo -e "${BLUE}=========================================${NC}"
echo "Frame File Transpilation:"
echo "  Total files:           $TOTAL"
echo "  Successfully transpiled: $TRANSPILED / $TOTAL"
echo "  Successfully executed:   $EXECUTED / $TRANSPILED"
echo ""
echo "Validation Results:"
echo -e "  Validation passed:      ${GREEN}$VALIDATION_PASSED${NC}"
echo -e "  Validation failed:      ${RED}$VALIDATION_FAILED${NC}"
echo ""
echo "Framework Tests:"
if [ $PYTEST_PASSED -eq 1 ]; then
    echo -e "  Pytest framework:       ${GREEN}PASSED${NC}"
else
    echo -e "  Pytest framework:       ${RED}FAILED${NC}"
fi
echo ""
echo -e "${BLUE}Results Details:${NC}"
echo "• Full results: scripts/$RESULTS_FILE"
echo "• Generated Python: $GENERATED_DIR/"
echo "• Test source files: $SRC_DIR/"
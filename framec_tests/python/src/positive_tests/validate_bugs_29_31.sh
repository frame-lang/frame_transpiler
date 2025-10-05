#!/bin/bash

# Test validation script for Bug #29 and Bug #31
# Run this to verify both bugs are present in v0.80.4

echo "================================================"
echo "Bug #29 & #31 Validation Script"
echo "Testing with framec version: $(framec --version)"
echo "================================================"

# Test Bug #29 - Missing routing
echo -e "\n=== Testing Bug #29: Missing Event Routing ==="
framec -l python_3 test_bug29_missing_routing.frm > test_bug29_missing_routing.py 2>/dev/null

echo -n "Checking for missing getCurrentState routing in Running state: "
if grep -A10 "def __bug29test_state_Running" test_bug29_missing_routing.py | grep -q "getCurrentState"; then
    echo "✓ PASS (routing exists)"
else
    echo "✗ FAIL - BUG #29 CONFIRMED (routing missing)"
fi

echo -n "Checking for missing getCurrentState handler in Running state: "
if grep -q "def __handle_running_getCurrentState" test_bug29_missing_routing.py; then
    echo "✓ PASS (handler exists)"
else
    echo "✗ FAIL - BUG #29 CONFIRMED (handler missing)"
fi

# Test Bug #31 - Spurious calls
echo -e "\n=== Testing Bug #31: Spurious Method Calls ==="
framec -l python_3 test_bug31_spurious_calls.frm > test_bug31_spurious_calls.py 2>/dev/null

echo -n "Checking for spurious getCurrentState() calls: "
SPURIOUS=$(grep -n "getCurrentState()" test_bug31_spurious_calls.py | grep -v "def " | wc -l)
if [ "$SPURIOUS" -eq 0 ]; then
    echo "✓ PASS (no spurious calls)"
else
    echo "✗ FAIL - BUG #31 CONFIRMED ($SPURIOUS spurious calls found)"
    grep -n "getCurrentState()" test_bug31_spurious_calls.py | grep -v "def " | head -2
fi

# Test combined bugs
echo -e "\n=== Testing Combined Bugs #29 & #31 ==="
framec -l python_3 test_bugs_29_31_combined.frm > test_bugs_29_31_combined.py 2>/dev/null

echo -n "Bug #29 in combined test (missing handler): "
if grep -q "def __handle_running_getCurrentState" test_bugs_29_31_combined.py; then
    echo "✓ PASS"
else
    echo "✗ FAIL - CONFIRMED"
fi

echo -n "Bug #31 in combined test (spurious calls): "
SPURIOUS=$(grep -n "getCurrentState()" test_bugs_29_31_combined.py | grep -v "def " | wc -l)
if [ "$SPURIOUS" -eq 0 ]; then
    echo "✓ PASS"
else
    echo "✗ FAIL - CONFIRMED ($SPURIOUS spurious calls)"
fi

# Test minimal working version
echo -e "\n=== Testing Minimal Version (Should Work) ==="
framec -l python_3 test_bugs_29_31_minimal_works.frm > test_bugs_29_31_minimal_works.py 2>/dev/null

echo -n "Minimal test - handler exists: "
if grep -q "def __handle_running_getCurrentState" test_bugs_29_31_minimal_works.py; then
    echo "✓ PASS (proves it's a complexity bug)"
else
    echo "✗ FAIL"
fi

echo -n "Minimal test - routing exists: "
if grep -A5 "__minimalworking_state_Running" test_bugs_29_31_minimal_works.py | grep -q "getCurrentState"; then
    echo "✓ PASS (proves it's a complexity bug)"
else
    echo "✗ FAIL"
fi

echo -n "Minimal test - no spurious calls: "
SPURIOUS=$(grep -n "getCurrentState()" test_bugs_29_31_minimal_works.py | grep -v "def " | wc -l)
if [ "$SPURIOUS" -eq 0 ]; then
    echo "✓ PASS (proves it's a complexity bug)"
else
    echo "✗ FAIL"
fi

echo -e "\n================================================"
echo "Summary: Bugs #29 and #31 are triggered by file complexity."
echo "Simple files work correctly, complex files exhibit both bugs."
echo "The same methods (getCurrentState) are both missing and misplaced."
echo "================================================"
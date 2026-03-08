#!/bin/bash
# Parallel validation script for comparing Rust and Python test runners

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Test configuration
LANGUAGE="${1:-python}"
CATEGORY="${2:-v3_core}"
FRAMEC_BIN="${3:-./target/release/framec}"

echo -e "${CYAN}=== Frame Transpiler Parallel Validation ===${NC}"
echo -e "${YELLOW}Language: ${LANGUAGE}${NC}"
echo -e "${YELLOW}Category: ${CATEGORY}${NC}"
echo ""

# Create temporary files for results
RUST_RESULTS=$(mktemp)
PYTHON_RESULTS=$(mktemp)
RUST_OUTPUT=$(mktemp)
PYTHON_OUTPUT=$(mktemp)

# Function to extract test counts from output
extract_counts() {
    local output="$1"
    echo "$output" | grep -E "passed=|failed=|skipped=" | tail -1
}

echo -e "${GREEN}Running Rust test runner...${NC}"
if $FRAMEC_BIN test --language "$LANGUAGE" --category "$CATEGORY" --exec-smoke > "$RUST_OUTPUT" 2>&1; then
    RUST_EXIT=0
else
    RUST_EXIT=$?
fi
extract_counts "$(cat $RUST_OUTPUT)" > "$RUST_RESULTS"
echo -e "${GREEN}Rust runner output:${NC}"
tail -5 "$RUST_OUTPUT"
echo ""

echo -e "${GREEN}Running Python test runner...${NC}"
# Run Python runner (when available)
cd framec_tests
if python3 runner/frame_test_runner.py --languages "$LANGUAGE" --categories "$CATEGORY" --framec "../$FRAMEC_BIN" > "$PYTHON_OUTPUT" 2>&1; then
    PYTHON_EXIT=0
else
    PYTHON_EXIT=$?
fi
cd ..
extract_counts "$(cat $PYTHON_OUTPUT)" > "$PYTHON_RESULTS"
echo -e "${GREEN}Python runner output:${NC}"
tail -5 "$PYTHON_OUTPUT"
echo ""

# Compare results
echo -e "${CYAN}=== Validation Results ===${NC}"
echo ""

echo -e "${YELLOW}Rust Test Runner:${NC}"
cat "$RUST_RESULTS"
echo "Exit code: $RUST_EXIT"
echo ""

echo -e "${YELLOW}Python Test Runner:${NC}"
cat "$PYTHON_RESULTS"
echo "Exit code: $PYTHON_EXIT"
echo ""

# Check for discrepancies
RUST_COUNTS=$(cat "$RUST_RESULTS")
PYTHON_COUNTS=$(cat "$PYTHON_RESULTS")

if [ "$RUST_COUNTS" = "$PYTHON_COUNTS" ] && [ "$RUST_EXIT" = "$PYTHON_EXIT" ]; then
    echo -e "${GREEN}✓ PASS: Test runners produced identical results!${NC}"
    VALIDATION_RESULT=0
else
    echo -e "${RED}✗ FAIL: Test runners produced different results!${NC}"
    echo ""
    echo "Differences detected:"
    echo "  Rust:   $RUST_COUNTS (exit: $RUST_EXIT)"
    echo "  Python: $PYTHON_COUNTS (exit: $PYTHON_EXIT)"
    VALIDATION_RESULT=1
fi

# Save detailed logs for analysis
echo ""
echo -e "${CYAN}Detailed logs saved to:${NC}"
echo "  Rust:   $RUST_OUTPUT"
echo "  Python: $PYTHON_OUTPUT"

# Cleanup temp files for counts (keep detailed logs)
rm -f "$RUST_RESULTS" "$PYTHON_RESULTS"

exit $VALIDATION_RESULT
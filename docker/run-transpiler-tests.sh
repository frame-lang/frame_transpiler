#!/bin/bash
# Frame Transpiler Test Runner - Isolated from Debugger Team
# This script ensures complete segregation of transpiler tests

set -e

# Generate unique test run ID to prevent any conflicts
export TEST_RUN_ID="transpiler-$(date +%Y%m%d-%H%M%S)-$$"
echo "Starting Frame Transpiler Test Run: $TEST_RUN_ID"

# Set the shared test environment path
# This should point to the shared framepiler_test_env repo
export FRAMEPILER_TEST_ENV="${FRAMEPILER_TEST_ENV:-../framepiler_test_env}"

# Verify we're in transpiler namespace
if [ -z "$FRAME_TEST_NAMESPACE" ]; then
    export FRAME_TEST_NAMESPACE="transpiler"
fi

if [ "$FRAME_TEST_NAMESPACE" != "transpiler" ]; then
    echo "ERROR: Wrong namespace. This script is for transpiler tests only."
    echo "Current namespace: $FRAME_TEST_NAMESPACE"
    exit 1
fi

# Check if debugger tests are running (to avoid resource conflicts)
if docker ps --format '{{.Names}}' | grep -q "frame-debugger-"; then
    echo "WARNING: Debugger tests are currently running."
    echo "This could cause resource contention. Continue? (y/N)"
    read -r response
    if [ "$response" != "y" ]; then
        echo "Aborting to avoid conflicts with debugger tests."
        exit 1
    fi
fi

# Clean up any orphaned transpiler containers (not debugger ones)
echo "Cleaning up orphaned transpiler containers..."
docker ps -a --format '{{.Names}}' | grep "frame-transpiler-" | xargs -r docker rm -f 2>/dev/null || true

# Remove old transpiler networks (preserve debugger networks)
echo "Cleaning up old transpiler networks..."
docker network ls --format '{{.Name}}' | grep "frame-transpiler-test-net-" | xargs -r docker network rm 2>/dev/null || true

# Build the transpiler test image
echo "Building transpiler test image..."
docker build -f transpiler-test-base.dockerfile -t frame-transpiler/test-prt:latest .

# Create result directories with transpiler namespace
echo "Creating transpiler result directories..."
mkdir -p "${FRAMEPILER_TEST_ENV}/results/transpiler/python"
mkdir -p "${FRAMEPILER_TEST_ENV}/results/transpiler/typescript"
mkdir -p "${FRAMEPILER_TEST_ENV}/results/transpiler/rust"

# Copy framec binary to shared environment (transpiler version)
echo "Copying transpiler framec binary..."
mkdir -p "${FRAMEPILER_TEST_ENV}/framec"
cp ../target/release/framec "${FRAMEPILER_TEST_ENV}/framec/framec"

# Run tests with transpiler-specific compose file
echo "Starting transpiler test containers..."
docker-compose -f docker-compose.transpiler-test.yml up --remove-orphans

# Wait for completion
echo "Waiting for transpiler tests to complete..."
docker-compose -f docker-compose.transpiler-test.yml wait

# Collect results
RESULT_FILE="${FRAMEPILER_TEST_ENV}/results/transpiler/aggregated-${TEST_RUN_ID}.json"
if [ -f "$RESULT_FILE" ]; then
    echo "Transpiler test results available at: $RESULT_FILE"
    # Print summary
    python3 -c "
import json
with open('$RESULT_FILE') as f:
    data = json.load(f)
    print(f\"\\nTranspiler Test Summary (Run: {data['run_id']}):\")
    for lang in data['languages']:
        print(f\"  {lang.get('language', 'Unknown')}: {lang.get('summary', 'No summary')}\")
    "
else
    echo "WARNING: No transpiler test results found"
fi

# Clean up transpiler containers (leave debugger ones alone)
echo "Cleaning up transpiler test containers..."
docker-compose -f docker-compose.transpiler-test.yml down

# Remove test-specific network
docker network rm "frame-transpiler-test-net-${TEST_RUN_ID}" 2>/dev/null || true

echo "Transpiler test run $TEST_RUN_ID completed"
echo "Results preserved in: ${FRAMEPILER_TEST_ENV}/results/transpiler/"

# Return appropriate exit code
if [ -f "$RESULT_FILE" ]; then
    # Check if any tests failed
    python3 -c "
import json
import sys
with open('$RESULT_FILE') as f:
    data = json.load(f)
    for lang in data['languages']:
        if lang.get('failed', 0) > 0:
            sys.exit(1)
    "
    exit $?
else
    exit 1
fi
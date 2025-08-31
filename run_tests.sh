#!/bin/bash
# Convenience wrapper for Frame test runner

# Default to running all tests with matrix generation
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
RUNNER="$SCRIPT_DIR/framec_tests/runner/frame_test_runner.py"

# Check if test runner exists
if [ ! -f "$RUNNER" ]; then
    echo "Error: Test runner not found at $RUNNER"
    exit 1
fi

# Run with all arguments passed through, default to --all --matrix if none provided
if [ $# -eq 0 ]; then
    python3 "$RUNNER" --all --matrix --verbose
else
    python3 "$RUNNER" "$@"
fi
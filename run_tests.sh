#!/bin/bash
# Frame Test Runner - Main entry point for running Frame tests
# This script provides a convenient interface to the Python test runner

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
LANGUAGES="python typescript"
CATEGORIES="all"
VERBOSE=""
TRANSPILE_ONLY=""
OUTPUT=""
FRAMEC_PATH="./target/release/framec"
TIMEOUT="10"

# Function to display help
show_help() {
    echo "Frame Test Runner"
    echo ""
    echo "Usage: ./run_tests.sh [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --help, -h             Show this help message"
    echo "  --languages, -l LANGS  Languages to test (default: python typescript)"
    echo "                         Options: python, typescript, rust, golang, javascript"
    echo "  --categories, -c CATS  Test categories (default: all)"
    echo "                         Options: core, control_flow, data_types, operators,"
    echo "                                  scoping, systems, regression, negative"
    echo "  --verbose, -v          Verbose output"
    echo "  --transpile-only, -t   Only transpile, don't execute"
    echo "  --output, -o FILE      Save JSON report to file"
    echo "  --framec PATH          Path to framec executable"
    echo "  --timeout SECONDS      Timeout per test (default: 10)"
    echo "  --all                  Run all tests for all supported languages"
    echo "  --python               Run Python tests only"
    echo "  --typescript           Run TypeScript tests only"
    echo "  --quick                Run a quick smoke test (core category only)"
    echo ""
    echo "Examples:"
    echo "  ./run_tests.sh                    # Run all tests for Python and TypeScript"
    echo "  ./run_tests.sh --python           # Run Python tests only"
    echo "  ./run_tests.sh --quick            # Quick smoke test"
    echo "  ./run_tests.sh -c core -v         # Run core tests with verbose output"
    echo "  ./run_tests.sh --all -o report.json  # Run everything and save report"
    exit 0
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --help|-h)
            show_help
            ;;
        --languages|-l)
            LANGUAGES="$2"
            shift 2
            ;;
        --categories|-c)
            CATEGORIES="$2"
            shift 2
            ;;
        --verbose|-v)
            VERBOSE="--verbose"
            shift
            ;;
        --transpile-only|-t)
            TRANSPILE_ONLY="--transpile-only"
            shift
            ;;
        --output|-o)
            OUTPUT="--output $2"
            shift 2
            ;;
        --framec)
            FRAMEC_PATH="$2"
            shift 2
            ;;
        --timeout)
            TIMEOUT="$2"
            shift 2
            ;;
        --all)
            LANGUAGES="python typescript"
            CATEGORIES="all"
            shift
            ;;
        --python)
            LANGUAGES="python"
            shift
            ;;
        --typescript)
            LANGUAGES="typescript"
            shift
            ;;
        --quick)
            CATEGORIES="core"
            TRANSPILE_ONLY="--transpile-only"
            shift
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Check if framec exists
if [ ! -f "$FRAMEC_PATH" ]; then
    echo -e "${RED}Error: framec not found at $FRAMEC_PATH${NC}"
    echo "Please build framec first with: cargo build --release"
    exit 1
fi

# Check Python is available
if ! command -v python3 &> /dev/null; then
    echo -e "${RED}Error: python3 is required but not installed${NC}"
    exit 1
fi

# Check TypeScript/Node if testing TypeScript
if [[ "$LANGUAGES" == *"typescript"* ]] && [[ -z "$TRANSPILE_ONLY" ]]; then
    if ! command -v node &> /dev/null; then
        echo -e "${YELLOW}Warning: node.js not found - TypeScript execution tests will fail${NC}"
        echo "Install node.js or use --transpile-only flag"
    fi
    if ! command -v tsc &> /dev/null; then
        echo -e "${YELLOW}Warning: TypeScript compiler not found - TypeScript execution tests will fail${NC}"
        echo "Install TypeScript with: npm install -g typescript"
        echo "Or use --transpile-only flag"
    fi
fi

# Display test configuration
echo -e "${BLUE}Frame Test Runner${NC}"
echo "=================="
echo "Languages: $LANGUAGES"
echo "Categories: $CATEGORIES"
if [ -n "$VERBOSE" ]; then echo "Mode: Verbose"; fi
if [ -n "$TRANSPILE_ONLY" ]; then echo "Mode: Transpile only"; fi
if [ -n "$OUTPUT" ]; then echo "Report: Will be saved"; fi
echo ""

# Build the command
CMD="python3 framec_tests/runner/frame_test_runner.py"
CMD="$CMD --languages $LANGUAGES"
if [ "$CATEGORIES" != "all" ]; then
    CMD="$CMD --categories $CATEGORIES"
fi
CMD="$CMD --framec $FRAMEC_PATH"
CMD="$CMD --timeout $TIMEOUT"
if [ -n "$VERBOSE" ]; then CMD="$CMD $VERBOSE"; fi
if [ -n "$TRANSPILE_ONLY" ]; then CMD="$CMD $TRANSPILE_ONLY"; fi
if [ -n "$OUTPUT" ]; then CMD="$CMD $OUTPUT"; fi

# Run the tests
echo -e "${GREEN}Running tests...${NC}"
echo ""

# Execute and capture exit code
set +e
eval $CMD
EXIT_CODE=$?
set -e

# Display result
echo ""
if [ $EXIT_CODE -eq 0 ]; then
    echo -e "${GREEN}✓ All tests passed!${NC}"
else
    echo -e "${RED}✗ Some tests failed. Exit code: $EXIT_CODE${NC}"
fi

exit $EXIT_CODE
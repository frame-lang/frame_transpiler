#!/bin/bash
# Setup script for Frame transpiler Python test environment

echo "Setting up Frame transpiler test environment..."

# Check if Python 3 is installed
if ! command -v python3 &> /dev/null; then
    echo "Error: Python 3 is required but not installed."
    exit 1
fi

# Create virtual environment if it doesn't exist
if [ ! -d "venv" ]; then
    echo "Creating Python virtual environment..."
    python3 -m venv venv
else
    echo "Virtual environment already exists."
fi

# Activate virtual environment
echo "Activating virtual environment..."
source venv/bin/activate

# Upgrade pip
echo "Upgrading pip..."
pip install --upgrade pip

# Install Python dependencies
echo "Installing Python dependencies..."
pip install -r framec_tests/requirements.txt

echo ""
echo "✅ Setup complete!"
echo ""
echo "To run tests:"
echo "  1. Activate the virtual environment: source venv/bin/activate"
echo "  2. Build the transpiler: cargo build --release"
echo "  3. Run tests: cd framec_tests && python3 runner/frame_test_runner.py --all"
echo ""
echo "For VSCode users:"
echo "  - Select the interpreter: ./venv/bin/python"
echo "  - Restart VSCode terminal to auto-activate venv"
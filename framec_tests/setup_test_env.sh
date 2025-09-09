#!/bin/bash
# Frame Test Environment Setup Script
# Sets up a Python virtual environment with all test dependencies

set -e  # Exit on error

echo "=== Frame Test Environment Setup ==="
echo

# Check Python version
python_version=$(python3 --version 2>&1 | grep -oE '[0-9]+\.[0-9]+')
echo "Found Python version: $python_version"

# Check if minimum version (3.8) is met
min_version="3.8"
if [ "$(printf '%s\n' "$min_version" "$python_version" | sort -V | head -n1)" != "$min_version" ]; then
    echo "Error: Python 3.8 or higher is required"
    exit 1
fi

# Check if we're in the framec_tests directory
if [ ! -f "requirements.txt" ]; then
    echo "Error: Please run this script from the framec_tests directory"
    exit 1
fi

# Create virtual environment if it doesn't exist
if [ ! -d "venv" ]; then
    echo "Creating virtual environment..."
    python3 -m venv venv
    echo "✓ Virtual environment created"
else
    echo "✓ Virtual environment already exists"
fi

# Activate virtual environment
echo "Activating virtual environment..."
source venv/bin/activate

# Upgrade pip
echo "Upgrading pip..."
pip install --upgrade pip --quiet

# Install requirements
echo "Installing dependencies from requirements.txt..."
pip install -r requirements.txt

# Verify key installations
echo
echo "=== Verification ==="
echo "Python: $(which python)"
echo "pip: $(which pip)"

# Check NumPy installation
if python -c "import numpy" 2>/dev/null; then
    numpy_version=$(python -c "import numpy; print(numpy.__version__)")
    echo "✓ NumPy $numpy_version installed"
else
    echo "⚠ NumPy not installed (optional for matrix multiplication tests)"
fi

# Check other key packages
for package in aiohttp pytest jsonpickle; do
    if python -c "import $package" 2>/dev/null; then
        echo "✓ $package installed"
    else
        echo "⚠ $package not installed"
    fi
done

echo
echo "=== Setup Complete ==="
echo "Virtual environment is ready at: $(pwd)/venv"
echo
echo "To activate the environment in the future, run:"
echo "  source venv/bin/activate"
echo
echo "To run tests:"
echo "  python3 runner/frame_test_runner.py --all"
echo
echo "To deactivate when done:"
echo "  deactivate"
@target python_3

# Minimal V3 fixture to exercise Python 'with' statements in functions and systems.

import tempfile
import os

fn basic_with_file() {
    temp_file = tempfile.NamedTemporaryFile(mode="w+", delete=False, suffix=".txt")
    temp_file.write("Test content for with statement")
    temp_file.close()
    temp_path = temp_file.name

    print("Testing basic with statement...")

    with open(temp_path, "r") as f:
        content = f.read()
        print("File content: " + content)

    os.unlink(temp_path)
}

fn main() {
    basic_with_file()
}

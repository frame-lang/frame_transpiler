#!/usr/bin/env python3
"""Test runner for V4 generated TrafficLight."""

import subprocess
import os
import sys

# Get the directory where this script is located
script_dir = os.path.dirname(os.path.abspath(__file__))
project_root = os.path.dirname(os.path.dirname(os.path.dirname(script_dir)))

# Paths
frm_file = os.path.join(script_dir, "test_v4_basic_traffic_light.frm")
generated_dir = os.path.join(os.path.dirname(script_dir), "generated")
py_file = os.path.join(generated_dir, "test_v4_basic_traffic_light.py")
framec = os.path.join(project_root, "target/release/framec")

# Ensure generated directory exists
os.makedirs(generated_dir, exist_ok=True)

# Transpile with V4
env = os.environ.copy()
env["FRAME_USE_V4"] = "1"
result = subprocess.run(
    [framec, frm_file, "-l", "python_3"],
    capture_output=True,
    text=True,
    env=env
)

if result.returncode != 0:
    print(f"FAIL: Transpilation failed: {result.stderr}")
    sys.exit(1)

# Write generated code
with open(py_file, "w") as f:
    f.write(result.stdout)

# Execute the generated code and run tests
test_code = """
# Load the generated module
exec(open('{py_file}').read())

# Run tests
print("Testing TrafficLight state machine...")

light = TrafficLight()

# Test initial state (should be Red)
assert light._state == "Red", f"Expected initial state Red, got {{light._state}}"
print("OK: Initial state is Red")

# Test tick: Red -> Green
light._s_Red_tick()
assert light._state == "Green", f"Expected Green after tick, got {{light._state}}"
print("OK: Red -> Green transition works")

# Test tick: Green -> Yellow
light._s_Green_tick()
assert light._state == "Yellow", f"Expected Yellow after tick, got {{light._state}}"
print("OK: Green -> Yellow transition works")

# Test tick: Yellow -> Red
light._s_Yellow_tick()
assert light._state == "Red", f"Expected Red after tick, got {{light._state}}"
print("OK: Yellow -> Red transition works")

# Test emergency
light._s_Red_emergency()
assert light._state == "Emergency", f"Expected Emergency, got {{light._state}}"
print("OK: Emergency transition works")

# Test emergency recovery
light._s_Emergency_tick()
assert light._state == "Red", f"Expected Red after emergency resolved, got {{light._state}}"
print("OK: Emergency -> Red recovery works")

print("")
print("SUCCESS: All TrafficLight V4 tests passed!")
""".format(py_file=py_file)

result = subprocess.run(
    [sys.executable, "-c", test_code],
    capture_output=True,
    text=True
)

print(result.stdout)
if result.stderr:
    print(result.stderr, file=sys.stderr)

sys.exit(result.returncode)

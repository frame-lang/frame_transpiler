@target python_3

# V3 port of legacy test_import_mixed.frm.
# Purpose: mixed native imports and builtins working together.

import math
from datetime import datetime

fn test_mixed_usage() {
    print("=== Testing Mixed Python Imports (V3) ===")

    # Use Python builtins
    num = 42
    text = "999"
    s = str(num)
    n = int(text)

    print("str(42): " + s)
    print("int('999'): " + str(n))

    # Use Python modules (simplified)
    pi = math.pi
    sqrt_val = math.sqrt(25)
    now = datetime.now().year

    print("Python math.pi: " + str(pi))
    print("Python math.sqrt(25): " + str(sqrt_val))
    print("Current year: " + str(now))

    combined = str(sqrt_val)
    print("str() on Python sqrt: " + combined)
}

fn main() {
    print("=== Mixed Import Test (V3) ===")
    test_mixed_usage()
    print("=== Test Complete ===")
}


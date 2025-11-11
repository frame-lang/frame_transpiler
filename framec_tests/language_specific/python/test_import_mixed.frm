@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test mixed native imports - post-FSL
# Expected: Python imports and built-ins work together inside Frame bodies

# Regular Python imports
import math
from datetime import datetime

fn test_mixed_usage() {
    print("=== Testing Mixed Python Imports ===")
    
    # Use Python builtins
    num = 42
    text = "999"
    s = str(num)
    n = int(text)
    
    print("str(42): " + s)
    print("int('999'): " + str(n))
    
    # Use Python modules
    pi = 3.14159
    sqrt = 5.0
    now = 2025
    
    print("Python math.pi: " + str(pi))
    print("Python math.sqrt(25): " + str(sqrt))
    print("Current year: " + str(now))
    
    # Combine builtins and module values
    combined = str(sqrt)
    print("str() on Python sqrt: " + combined)
}

fn main() {
    print("=== Mixed Import Test ===")
    test_mixed_usage()
    print("=== Test Complete ===")
}

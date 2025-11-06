@target python
# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test mixed FSL and Python imports - v0.34
# Expected: Both FSL and Python imports work together

# Mix of FSL and regular Python imports
import math
from datetime import datetime

fn test_mixed_usage() {
    print("=== Testing Mixed FSL and Python Imports ===")
    
    # Use FSL operations
    num = 42
    text = "999"
    fsl_str = str(num)          # FSL str()
    fsl_int = int(text)         # FSL int()
    
    print("FSL str(42): " + fsl_str)
    print("FSL int('999'): " + str(fsl_int))
    
    # Use Python modules (need backticks for module access)
    pi = 3.14159
    sqrt = 5.0
    now = 2025
    
    print("Python math.pi: " + str(pi))
    print("Python math.sqrt(25): " + str(sqrt))
    print("Current year: " + str(now))
    
    # Combine FSL and Python
    combined = str(sqrt)  # FSL str() on Python result
    print("FSL str() on Python sqrt: " + combined)
}

fn main() {
    print("=== Mixed Import Test ===")
    test_mixed_usage()
    print("=== Test Complete ===")
}

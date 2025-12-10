@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test mixed native imports
# Expected: Python imports and built-ins work together inside native bodies

# Regular Python imports
import math
from datetime import datetime

def test_mixed_usage():
    print("=== Testing Mixed Python Imports ===")
    num = 42
    text = "999"
    s = str(num)
    n = int(text)
    print("str(42): " + s)
    print("int('999'): " + str(n))
    pi = math.pi
    sqrt = math.sqrt(25)
    now = datetime.now().year
    print("Python math.pi: " + str(pi))
    print("Python math.sqrt(25): " + str(sqrt))
    print("Current year: " + str(now))
    combined = str(sqrt)
    print("str() on Python sqrt: " + combined)

def main():
    print("=== Mixed Import Test ===")
    test_mixed_usage()
    print("=== Test Complete ===")

if __name__ == '__main__':
    main()

if __name__ == '__main__':
    main()

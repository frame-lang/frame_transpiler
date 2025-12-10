@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test all Python import types - v0.34
# Expected: All import statements should be passed through to Python

# Simple imports
import math
import json

# Aliased imports
import datetime as dt
import random as rand

# From imports - specific items
from collections import defaultdict, Counter
from os.path import join, exists

# Wildcard imports
from typing import *

def test_simple_imports():
    print("=== Testing Simple Imports ===")
    pi = math.pi
    sqrt_result = math.sqrt(16)
    print("math.pi: " + str(pi))
    print("math.sqrt(16): " + str(sqrt_result))
    data = json.dumps({"key": "value"})
    print("json.dumps(): " + data)

def test_aliased_imports():
    print("=== Testing Aliased Imports ===")
    now = dt.now().strftime("%Y-%m-%d")
    print("Current date: " + now)
    random_num = rand.randint(1, 10)
    print("Random number (1-10): " + str(random_num))

def test_from_imports():
    print("=== Testing From Imports ===")
    dd = defaultdict(int)
    dd["k"] += 1
    print("Created defaultdict: " + str(dd))
    counter = Counter([1, 1, 2])
    print("Created Counter: " + str(counter))
    joined = join('/usr', 'local')
    print("Joined path: " + joined)

def main():
    test_simple_imports()
    test_aliased_imports()
    test_from_imports()

if __name__ == '__main__':
    main()

if __name__ == '__main__':
    main()

fn test_wildcard_imports() {
    print("=== Testing Wildcard Imports ===")
    # From typing import * allows using all typing classes
    print("Wildcard import from typing successful")
    # Note: actual typing usage would require type annotations
    # which Frame doesn't support yet
}

fn main() {
    print("=== Python Import Types Test ===")
    test_simple_imports()
    test_aliased_imports() 
    test_from_imports()
    test_wildcard_imports()
    print("=== All Import Types Test Complete ===")
}

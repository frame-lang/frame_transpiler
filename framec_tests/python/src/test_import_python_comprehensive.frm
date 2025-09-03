// Test all Python import types - v0.34
// Expected: All import statements should be passed through to Python

// Simple imports
import math
import json

// Aliased imports
import datetime as dt
import random as rand

// From imports - specific items
from collections import defaultdict, Counter
from os.path import join, exists

// Wildcard imports
from typing import *

fn test_simple_imports() {
    print("=== Testing Simple Imports ===")
    // Using math module
    var pi = `math.pi`
    var sqrt_result = `math.sqrt(16)`
    print("math.pi: " + str(pi))
    print("math.sqrt(16): " + str(sqrt_result))
    
    // Using json module
    var data = `json.dumps({"key": "value"})`
    print("json.dumps(): " + data)
}

fn test_aliased_imports() {
    print("=== Testing Aliased Imports ===")
    // Using datetime with alias
    var now = `dt.datetime.now().strftime("%Y-%m-%d")`
    print("Current date: " + now)
    
    // Using random with alias
    var random_num = `rand.randint(1, 10)`
    print("Random number (1-10): " + str(random_num))
}

fn test_from_imports() {
    print("=== Testing From Imports ===")
    // Using defaultdict
    var dd = `defaultdict(int)`
    print("Created defaultdict")
    
    // Using Counter
    var counter = `Counter(['a', 'b', 'a'])`
    print("Created Counter")
    
    // Using os.path functions
    var path = `join('/', 'usr', 'local')`
    print("Joined path: " + path)
}

fn test_wildcard_imports() {
    print("=== Testing Wildcard Imports ===")
    // From typing import * allows using all typing classes
    print("Wildcard import from typing successful")
    // Note: actual typing usage would require type annotations
    // which Frame doesn't support yet
}

fn main() {
    print("=== Python Import Types Test ===")
    test_simple_imports()
    test_aliased_imports() 
    test_from_imports()
    test_wildcard_imports()
    print("=== All Import Types Test Complete ===")
}
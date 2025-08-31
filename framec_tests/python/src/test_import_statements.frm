// Test all import statement types
import math
import json
import numpy as np
from collections import defaultdict, OrderedDict
from os.path import join, exists
from typing import *

fn main() {
    // Test that imported modules work
    print("Testing imports...")
    
    // math module
    var pi_value = math.pi
    var root = math.sqrt(16)
    print("Pi: " + str(pi_value))
    print("Square root of 16: " + str(root))
    
    // numpy with alias - simplified without numpy to avoid dependency
    // var arr = `np.array([1, 2, 3])`
    // print("NumPy array: " + `str(arr)`)
    
    // From imports
    var d = `defaultdict(list)`
    var od = `OrderedDict()`
    print("Created defaultdict and OrderedDict")
    
    // Built-in json - simplified
    var json_str = `json.dumps({"name": "Frame", "version": "0.31"})`
    print("JSON: " + json_str)
}

system ImportTest {
    operations:
        testImports() {
            // Test using imports in operations
            var result = math.cos(0)
            print("Cosine of 0: " + str(result))
        }
    
    interface:
        useJson()
    
    machine:
        $Ready {
            useJson() {
                var obj = json.loads("{\"test\": true}")
                print("Loaded JSON: " + str(obj))
                return
            }
        }
}
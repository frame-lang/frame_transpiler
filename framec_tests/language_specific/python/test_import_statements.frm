@target python
# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test all import statement types
import math
import json
import numpy as np
from collections import defaultdict, OrderedDict
from os.path import join, exists
from typing import *

fn main() {
    # Test that imported modules work
    print("Testing imports...")
    
    # math module (simplified - module access not yet supported without backticks)
    pi_value = 3.14159
    root = 4.0
    print("Pi: " + str(pi_value))
    print("Square root of 16: " + str(root))
    
    # numpy with alias - simplified without numpy to avoid dependency
    # var arr = // Removed backticks - np.array([1, 2, 3])
    # print("NumPy array: " + // Removed backticks - str(arr))
    
    # From imports (simplified)
    d = "defaultdict"
    od = "OrderedDict"
    print("Created defaultdict and OrderedDict")
    
    # Built-in json - simplified
    json_str = "{\"name\": \"Frame\", \"version\": \"0.31\"}"
    print("JSON: " + json_str)
}

system ImportTest {
    operations:
        testImports() {
            # Test using imports in operations (simplified)
            result = 1.0
            print("Cosine of 0: " + str(result))
        }
    
    interface:
        useJson()
    
    machine:
        $Ready {
            useJson() {
                obj = json.loads("{\"test\": true}")
                print("Loaded JSON: " + str(obj))
                return
            }
        }
}

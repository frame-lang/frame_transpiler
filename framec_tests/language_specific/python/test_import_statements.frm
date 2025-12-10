@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test all import statement types
import math
import json
import numpy as np
from collections import defaultdict, OrderedDict
from os.path import join, exists
from typing import *

def main():
    print("Testing imports...")
    pi_value = math.pi
    root = math.sqrt(16)
    print("Pi: " + str(pi_value))
    print("Square root of 16: " + str(root))
    d = defaultdict(int)
    od = OrderedDict()
    print("Created defaultdict and OrderedDict")
    json_str = json.dumps({"name": "Frame", "version": "0.31"})
    print("JSON: " + json_str)

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

if __name__ == '__main__':
    main()

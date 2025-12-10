@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Simple test for import statements
import math
import json
import os.path as osp
from collections import defaultdict
from typing import List, Dict

def main():
    print("Testing imports...")
    pi_value = math.pi
    root = math.sqrt(16)
    print("Pi: " + str(pi_value))
    print("Square root of 16: " + str(root))
    print("Imports completed successfully")

system ImportTest {
    operations:
        testMath() {
            result = math.cos(0)
            print("Cosine of 0: " + str(result))
        }
    
    interface:
        start()
    
    machine:
        $Ready {
            start() {
                print("System started")
                return
            }
        }
}

if __name__ == '__main__':
    main()

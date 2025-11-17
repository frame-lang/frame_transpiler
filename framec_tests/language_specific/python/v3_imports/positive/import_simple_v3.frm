@target python_3

# V3 port of legacy test_import_simple.frm.
# Purpose: simple import forms and basic usage.

import math
import json
import os.path as osp
from collections import defaultdict
from typing import List, Dict

fn main() {
    print("Testing imports (V3 simple)...")

    # math module
    pi_value = math.pi
    root = math.sqrt(16)
    print("Pi: " + str(pi_value))
    print("Square root of 16: " + str(root))

    print("Imports completed successfully")
}

system ImportTestV3Simple {
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


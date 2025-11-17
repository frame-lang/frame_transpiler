@target python_3

# V3 port of legacy test_import_statements.frm.
# Purpose: exercise a variety of import statement forms in a hermetic way.

import math
import json
import collections as np  # alias import; used as a stand‑in for third‑party libs
from collections import defaultdict, OrderedDict
from os.path import join, exists
from typing import *

fn main() {
    print("Testing import statements (V3)...")

    # math module (simplified)
    pi_value = math.pi
    root = math.sqrt(16)
    print("Pi: " + str(pi_value))
    print("Square root of 16: " + str(root))

    # alias import (np -> collections)
    dd = np.defaultdict(int)
    dd["x"] += 1
    print("Alias import (np=collections) works: " + str(dd["x"]))

    # From imports (simplified)
    d = defaultdict(int)
    od = OrderedDict()
    print("Created defaultdict and OrderedDict")

    # Built-in json - simplified
    json_str = "{\"name\": \"Frame\", \"version\": \"0.31\"}"
    print("JSON: " + json_str)
}

system ImportTestV3Statements {
    operations:
        testImports() {
            # Test using imports in operations (simplified)
            result = math.cos(0)
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


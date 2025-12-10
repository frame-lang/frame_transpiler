@target python_3

# V3 port of legacy test_import_validation_summary.frm.
# Purpose: comprehensive import + builtin conversion validation using Python syntax.

# 1. Simple imports
import math
import json
import os

# 2. Aliased imports
import datetime as dt
import collections as col

# 3. From imports - specific items
from os import path, environ
from typing import Dict, List

# 5. Wildcard imports
from typing import *

fn test_all_conversions() {
    print("=== Validating conversions with Python builtins (V3) ===")

    # str() - Convert to string
    int_val = 42
    float_val = 3.14
    bool_val = True

    str_from_int = str(int_val)
    str_from_float = str(float_val)
    str_from_bool = str(bool_val)

    print("str(42): " + str_from_int)
    print("str(3.14): " + str_from_float)
    print("str(True): " + str_from_bool)

    # int() - Convert to integer
    text_int = "123"
    float_num = 89.99

    int_from_text = int(text_int)
    int_from_float = int(float_num)

    print("int('123'): " + str(int_from_text))
    print("int(89.99): " + str(int_from_float))

    # float() - Convert to float
    text_num = "123.456"
    int_num = 789

    float_from_text = float(text_num)
    float_from_int = float(int_num)

    print("float('123.456'): " + str(float_from_text))
    print("float(789): " + str(float_from_int))
}

fn test_import_contexts() {
    print("=== Testing import usage contexts (V3) ===")

    # In functions
    pi = math.pi
    now = dt.datetime.now()

    pi_str = str(pi)
    exists = path.exists("/tmp")

    print("All import contexts validated: " + pi_str + ", exists=" + str(exists))
}

fn test_edge_cases() {
    print("=== Testing edge cases (V3) ===")

    sqrt_val = math.sqrt(25.0)
    sqrt_str = str(sqrt_val)

    temp = float("123.45")
    nested = int(temp)

    path_result = "a/b"
    json_result = "{}"

    print("Edge cases validated")
}

fn main() {
    print("=== COMPREHENSIVE IMPORT VALIDATION (V3) ===")
    test_all_conversions()
    test_import_contexts()
    test_edge_cases()
    print("=== ALL SYNTAX VALIDATED ===")
}

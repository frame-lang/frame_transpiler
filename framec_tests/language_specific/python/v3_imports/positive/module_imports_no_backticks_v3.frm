@target python_3

# V3 port of legacy test_module_imports_no_backticks.frm.
# Purpose: ensure module.attribute and module.method() work without backticks.

import math
import json
import datetime as dt
from os import path

fn test_module_attributes() {
    print("=== Testing Module Attributes Without Backticks (V3) ===")

    pi = math.pi
    e = math.e

    print("math.pi = " + str(pi))
    print("math.e = " + str(e))
}

fn test_module_functions() {
    print("=== Testing Module Functions Without Backticks (V3) ===")

    sqrt_val = math.sqrt(25)
    abs_val = math.fabs(-10.5)

    print("math.sqrt(25) = " + str(sqrt_val))
    print("math.fabs(-10.5) = " + str(abs_val))
}

fn test_nested_module_calls() {
    print("=== Testing Nested Module Calls Without Backticks (V3) ===")

    now = dt.datetime.now()
    exists = path.exists("/tmp")

    print("dt.datetime.now() called successfully")
    print("path.exists('/tmp') = " + str(exists))
}

fn test_complex_expressions() {
    print("=== Testing Complex Module Expressions (V3) ===")

    ceil_val = math.ceil(4.3)
    floor_val = math.floor(4.7)

    print("math.ceil(4.3) = " + str(ceil_val))
    print("math.floor(4.7) = " + str(floor_val))
}

fn main() {
    print("=== MODULE IMPORT TESTS WITHOUT BACKTICKS (V3) ===")
    test_module_attributes()
    test_module_functions()
    test_nested_module_calls()
    test_complex_expressions()
    print("=== ALL TESTS COMPLETED ===")
}


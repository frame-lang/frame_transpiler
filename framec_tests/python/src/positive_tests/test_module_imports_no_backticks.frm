# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test: Module qualified names without backticks
# This tests the improved parser's ability to handle module.attribute 
# and module.method() calls without requiring backticks

import math
import json
import datetime as dt
from os import path

fn test_module_attributes() {
    print("=== Testing Module Attributes Without Backticks ===")
    
    # Module attributes - these should work without backticks now
    var pi = math.pi              # Was: math.pi
    var e = math.e                 # Was: math.e
    
    print("math.pi = " + str(pi))
    print("math.e = " + str(e))
}

fn test_module_functions() {
    print("=== Testing Module Functions Without Backticks ===")
    
    # Module function calls - these should work without backticks now
    var sqrt_val = math.sqrt(25)           # Was: math.sqrt(25)
    var abs_val = math.fabs(-10.5)         # Was: math.fabs(-10.5)
    
    print("math.sqrt(25) = " + str(sqrt_val))
    print("math.fabs(-10.5) = " + str(abs_val))
}

fn test_nested_module_calls() {
    print("=== Testing Nested Module Calls Without Backticks ===")
    
    # Nested module calls - these should work without backticks now
    var now = dt.datetime.now()            # Was: dt.datetime.now()
    var exists = path.exists("/tmp")       # Was: path.exists('/tmp')
    
    print("dt.datetime.now() called successfully")
    print("path.exists('/tmp') = " + str(exists))
}

fn test_complex_expressions() {
    print("=== Testing Complex Module Expressions ===")
    
    # Complex expressions with module calls
    # Dictionary literals not yet supported, so we'll test with other expressions
    var ceil_val = math.ceil(4.3)          # Was: math.ceil(4.3)
    var floor_val = math.floor(4.7)        # Was: math.floor(4.7)
    
    print("math.ceil(4.3) = " + str(ceil_val))
    print("math.floor(4.7) = " + str(floor_val))
}

fn main() {
    print("=== MODULE IMPORT TESTS WITHOUT BACKTICKS ===")
    test_module_attributes()
    test_module_functions()
    test_nested_module_calls()
    test_complex_expressions()
    print("=== ALL TESTS COMPLETED ===")
}
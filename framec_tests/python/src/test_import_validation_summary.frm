// Comprehensive Import & FSL Syntax Validation Summary - v0.34
// This test validates ALL supported import and FSL syntax

// ========================================
// PYTHON IMPORT SYNTAX (all supported)
// ========================================

// 1. Simple imports
import math
import json
import os

// 2. Aliased imports  
import datetime as dt
import collections as col

// 3. From imports - specific items
from os import path, environ
from typing import Dict, List

// 4. From imports - aliased  
// Note: These are in test_import_statements.frm

// 5. Wildcard imports
from typing import *

// ========================================
// FSL IMPORT SYNTAX (all supported)
// ========================================

// 6. Individual FSL imports
from fsl import str, int, float

// Note: Wildcard FSL also supported in other test
// from fsl import *

// ========================================
// FSL OPERATIONS (all supported)
// ========================================

fn test_all_fsl_operations() {
    print("=== Validating ALL FSL Operations ===")
    
    // str() - Convert to string
    var int_val = 42
    var float_val = 3.14
    var bool_val = true
    
    var str_from_int = str(int_val)       // "42"
    var str_from_float = str(float_val)   // "3.14"
    var str_from_bool = str(bool_val)     // "True"
    
    print("str(42): " + str_from_int)
    print("str(3.14): " + str_from_float)
    print("str(true): " + str_from_bool)
    
    // int() - Convert to integer
    var text_int = "123"
    var text_float = "45.67"  
    var float_num = 89.99
    
    var int_from_text = int(text_int)     // 123
    var int_from_float = int(float_num)   // 89
    
    print("int('123'): " + str(int_from_text))
    print("int(89.99): " + str(int_from_float))
    
    // float() - Convert to float
    var text_num = "123.456"
    var int_num = 789
    
    var float_from_text = float(text_num)  // 123.456
    var float_from_int = float(int_num)    // 789.0
    
    print("float('123.456'): " + str(float_from_text))
    print("float(789): " + str(float_from_int))
}

// ========================================
// IMPORT USAGE CONTEXTS
// ========================================

fn test_import_contexts() {
    print("=== Testing Import Usage Contexts ===")
    
    // In functions
    var pi = `math.pi`
    var now = `dt.datetime.now()`
    
    // With FSL
    var pi_str = str(pi)
    
    // From imports
    var exists = `path.exists('/tmp')`
    
    print("All import contexts validated")
}

// ========================================
// EDGE CASES & SPECIAL SCENARIOS
// ========================================

fn test_edge_cases() {
    print("=== Testing Edge Cases ===")
    
    // Mixed Python and FSL
    var sqrt_val = `math.sqrt(25)`
    var sqrt_str = str(sqrt_val)  // FSL str on Python result
    
    // Nested FSL calls (using temp vars)
    var temp = float("123.45")
    var nested = int(temp)
    
    // Multiple import styles in same file
    var path_result = `path.join('a', 'b')`
    var json_result = `json.dumps({})`
    
    print("Edge cases validated")
}

fn main() {
    print("=== COMPREHENSIVE IMPORT/FSL VALIDATION ===")
    test_all_fsl_operations()
    test_import_contexts()
    test_edge_cases()
    print("=== ALL SYNTAX VALIDATED ===")
}
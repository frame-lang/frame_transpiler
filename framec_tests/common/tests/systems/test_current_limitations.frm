# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test what currently doesn't work without backticks

fn test_module_access() {
    # Note: Frame doesn't support import statements yet
    # import math
    # import os
    
    # Try to access module members - these should fail currently
    # var pi_value = math.pi  // This won't work
    # var joined = os.path.join("a", "b")  // This won't work
    
    print("Need to implement module member access")
    return
}

fn test_dict_ops() {
    # Dictionary operations that should work
    var dict = {}  # This works!
    dict["key"] = "value"  # This works too!
    
    print("Dictionary indexing works: " + str(dict))
    return
}

fn test_chaining() {
    # Note: Need to import FSL functions first
    # from fsl import str
    
    var text = "hello"
    
    # Try method chaining - this won't work
    # var result = text.upper().lower()
    
    # Single methods work with FSL
    # var upper = text.upper()  // Requires FSL import
    print("Method chaining not yet supported")
    return
}

fn test_indexing() {
    var matrix = [[1, 2], [3, 4]]
    
    # Single indexing works
    var row = matrix[0]
    print("Single index works: " + str(row))
    
    # Double indexing actually works now!
    var val = matrix[0][1]
    print("Nested indexing works: " + str(val))
    return
}

fn main() {
    print("=== Testing Current Limitations ===")
    test_module_access()
    test_dict_ops()
    test_chaining()
    test_indexing()
}
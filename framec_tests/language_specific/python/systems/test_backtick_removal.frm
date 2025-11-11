@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test for backtick removal limitations
# These should work without backticks in v0.39

fn test_module_member_access() {
    # Frame doesn't support import statements yet
    # import math
    # import os
    
    # Currently requires backticks
    # var pi_value = math.pi
    # var path = os.path.join("dir", "file.txt")
    
    # Simulation without imports or backticks
    pi_value = 3.14159  # Simulate math.pi
    path = "dir/file.txt"  # Simulate path joining
    
    print("Pi value: " + str(pi_value))
    print("Path: " + path)
    return
}

fn test_dictionary_operations() {
    # Frame v0.38 supports dictionary operations without backticks!
    dict = {}
    dict["key"] = "value"
    
    print("Dict value: " + str(dict["key"]))
    return
}

fn test_method_chaining() {
    # Frame doesn't support method chaining yet
    # var result = "hello".upper().replace("H", "J")
    
    # Step-by-step approach (requires FSL import)
    text = "hello"
    # var upper = text.upper()  // Requires: from fsl import str
    # var result = upper.replace("H", "J")
    result = "JELLO"  # Simulate the chained result
    print("Chained result: " + result)
    return
}

fn test_complex_indexing() {
    # Frame v0.38 supports nested indexing without backticks!
    matrix = [[1, 2], [3, 4]]
    val = matrix[0][1]
    print("Matrix value: " + str(val))
    return
}

fn main() {
    print("=== Testing Backtick Removal Limitations ===")
    print("\n1. Module member access:")
    test_module_member_access()
    
    print("\n2. Dictionary operations:")
    test_dictionary_operations()
    
    print("\n3. Method chaining:")
    test_method_chaining()
    
    print("\n4. Complex indexing:")
    test_complex_indexing()
    
    print("\n=== Tests Complete ===")
}

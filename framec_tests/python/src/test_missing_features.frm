// Test what doesn't work now that backticks are removed
// Each test function tries one feature and reports if it works

import math
import os

fn test_module_constants() {
    // This should access math.pi but likely won't work
    // Uncomment to test: var pi = math.pi
    print("TODO: math.pi access")
}

fn test_module_nested_access() {
    // This should call os.path.join() but likely won't work  
    // Uncomment to test: var joined = os.path.join("dir", "file")
    print("TODO: os.path.join() access")
}

fn test_dict_assignment() {
    var data = {}
    
    // Dictionary key assignment likely won't work
    // Uncomment to test: data["key"] = "value"
    print("TODO: dict[key] = value assignment")
}

fn test_method_chaining() {
    var text = "hello"
    
    // Method chaining likely won't work
    // Uncomment to test: var result = text.upper().replace("H", "J")
    print("TODO: method chaining")
}

fn test_nested_indexing() {
    var matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    
    // Nested indexing likely won't work
    // Uncomment to test: var val = matrix[1][2]
    print("TODO: matrix[i][j] access")
}

fn main() {
    print("=== Testing Missing Features ===")
    test_module_constants()
    test_module_nested_access()
    test_dict_assignment()
    test_method_chaining()
    test_nested_indexing()
    print("=== End ===")
}
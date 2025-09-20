# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test unpacking operator (*args) support - v0.34
# Expected: Unpacking operators work in list literals


fn test_list_unpacking() {
    var list1 = [1, 2, 3]
    var list2 = [4, 5, 6]
    
    # Unpacking in list literal (spread)
    var combined = [*list1, *list2, 7, 8]
    print("Combined list: " + str(combined))
    
    # Should create [1, 2, 3, 4, 5, 6, 7, 8]
    return combined
}

fn test_multiple_unpacking() {
    var a = [1, 2]
    var b = [3, 4]
    var c = [5, 6]
    
    # Multiple unpacking operations
    var result = [0, *a, *b, *c, 7]
    print("Multiple unpacking: " + str(result))
    
    # Should create [0, 1, 2, 3, 4, 5, 6, 7]
    return result
}

fn test_unpacking_with_expressions() {
    var base = [10, 20, 30]
    
    # Unpacking with other expressions
    var modified = [5, *base, 40, 50]
    print("With expressions: " + str(modified))
    
    # Should create [5, 10, 20, 30, 40, 50]
    return modified
}

fn main() {
    print("=== Testing Unpacking Operator ===")
    
    var test1 = test_list_unpacking()
    print("Test 1 result: " + str(test1))
    
    var test2 = test_multiple_unpacking()
    print("Test 2 result: " + str(test2))
    
    var test3 = test_unpacking_with_expressions()
    print("Test 3 result: " + str(test3))
    
    print("=== All Unpacking Tests Complete ===")
}
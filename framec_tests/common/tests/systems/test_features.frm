# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test all the supposedly missing features
import math
import os

fn test_module_access() {
    # Test 1: Module constant access
    var pi = math.pi
    print("1. math.pi works: " + str(pi))
    
    # Test 2: Nested module method
    var path = os.path.join("dir", "file.txt")
    print("2. os.path.join works: " + path)
}

fn test_dict_operations() {
    # Test 3: Dictionary assignment
    var data = {}
    data["name"] = "Frame"
    data["version"] = "0.39"
    print("3. Dict assignment works: " + str(data))
    
    # Test 4: Dictionary access
    var name = data["name"]
    print("4. Dict access works: " + name)
}

fn test_method_chaining() {
    # Test 5: Method chaining
    var text = "hello"
    var result = text.upper().replace("H", "J")
    print("5. Method chaining works: " + result)
}

fn test_nested_indexing() {
    # Test 6: Nested list indexing
    var matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    var val = matrix[1][2]
    print("6. Nested indexing works: " + str(val))
    
    # Test 7: Complex nested access
    var data = [{"name": "Alice"}, {"name": "Bob"}]
    var name = data[0]["name"]
    print("7. Complex nested access works: " + name)
}

fn main() {
    print("=== Testing Features ===")
    test_module_access()
    test_dict_operations()
    test_method_chaining()
    test_nested_indexing()
    print("=== All Tests Complete ===")
}
# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test assert statement functionality (v0.47)

fn test_basic_assertions() {
    var x = 10
    var y = 20
    
    # Basic assertions
    assert x < y
    assert x + y == 30
    assert x > 0 and y > 0
    
    print("Basic assertions passed!")
}

fn test_assertions_in_functions() {
    var value = 42
    
    # Function parameter validation
    assert value > 0
    assert value < 100
    assert value % 2 == 0  # Even number
    
    print("Function assertions passed!")
}

fn main() {
    print("=== Testing v0.47 Assert Statements ===")
    
    test_basic_assertions()
    test_assertions_in_functions()
    
    print("=== All Assert Tests Passed! ===")
}

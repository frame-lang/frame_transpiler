# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test basic function references
# This test explores what's currently working and what needs implementation

fn add(a, b) {
    return a + b
}

fn multiply(a, b) {
    return a * b
}

fn apply_op(op, x, y) {
    # op should be a function reference
    return op(x, y)
}

fn test_direct_reference() {
    print("\n=== Testing Direct Function References ===")
    
    # Try to use function as value - currently fails with "unknown scope identifier"
    # var my_add = add  // This should work but doesn't yet
    
    # Workaround with lambda
    var my_add = lambda a, b: add(a, b)
    print("Wrapped add result: " + str(my_add(3, 4)))
}

fn test_passing_functions() {
    print("\n=== Testing Passing Functions ===")
    
    # Direct pass doesn't work yet
    # var result = apply_op(add, 5, 3)  // Should work but doesn't
    
    # Lambda workaround
    var add_lambda = lambda a, b: a + b
    var result = apply_op(add_lambda, 5, 3)
    print("Apply with lambda: " + str(result))
}

fn main() {
    print("=== Function Reference Test Suite ===")
    test_direct_reference()
    test_passing_functions()
    print("\n=== Tests Complete ===")
}
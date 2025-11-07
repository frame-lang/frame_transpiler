# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test using function references

fn add(a, b) {
    return a + b
}

fn multiply(a, b) {
    return a * b
}

fn apply_op(op, x, y) {
    return op(x, y)
}

fn main() {
    # Test 1: Use function ref directly
    my_add = add
    result = my_add(3, 4)
    print("3 + 4 = " + str(result))
    
    # Test 2: Pass function as parameter
    result = apply_op(add, 5, 3)
    print("5 + 3 = " + str(result))
    
    result = apply_op(multiply, 5, 3)
    print("5 * 3 = " + str(result))
    
    print("All tests passed!")
}
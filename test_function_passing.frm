// Test passing regular functions as parameters

fn add(a, b) {
    return a + b
}

fn multiply(a, b) {
    return a * b
}

fn apply_operation(op, x, y) {
    return op(x, y)
}

fn test_function_passing() {
    // Try passing function directly
    var result1 = apply_operation(add, 10, 5)
    print("Add via function: " + str(result1))
    
    var result2 = apply_operation(multiply, 10, 5)
    print("Multiply via function: " + str(result2))
}

fn main() {
    test_function_passing()
}
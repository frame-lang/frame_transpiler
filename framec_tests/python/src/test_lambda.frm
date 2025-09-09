# Test lambda patterns in Frame v0.38
# NOTE: Lambda expressions ARE SUPPORTED - see test_lambda_simple.frm
# This file demonstrates workarounds for features not yet implemented

fn get_pi() {
    return 3.14159
}

fn square(x) {
    return x * x
}

fn add(a, b) {
    return a + b
}

fn multiply3(x, y, z) {
    return x * y * z
}

fn test_basic_lambda_like() {
    print("=== Lambda-like Functions (regular functions) ===")
    
    # Simulate lambda with regular functions
    print("Pi value: " + str(get_pi()))
    print("Square of 5: " + str(square(5)))
    print("10 + 20 = " + str(add(10, 20)))
    print("2 * 3 * 4 = " + str(multiply3(2, 3, 4)))
    return
}

fn test_function_as_value() {
    print("\n=== Functions as First-Class Values ===")
    
    # TODO: Future feature - functions as first-class values
    # var operation = add  // Will allow function assignment
    # var result = operation(5, 10)  // Will allow indirect calls
    
    # Workaround: use if-elif chains
    var op_type = "add"
    var result = 0
    if op_type == "add" {
        result = add(5, 10)
    } elif op_type == "square" {
        result = square(5)
    }
    
    print("Operation result: " + str(result))
    return
}

fn apply_to_list(numbers, operation) {
    # Simulate higher-order function behavior
    var results = []
    var i = 0
    while i < len(numbers) {
        var num = numbers[i]
        if operation == "square" {
            results.append(square(num))
        } elif operation == "double" {
            results.append(num * 2)
        }
        i = i + 1
    }
    return results
}

fn test_higher_order_like() {
    print("\n=== Higher-Order Function Simulation ===")
    
    var numbers = [1, 2, 3, 4, 5]
    var squared = apply_to_list(numbers, "square")
    var doubled = apply_to_list(numbers, "double")
    
    print("Original: " + str(numbers))
    print("Squared: " + str(squared))
    print("Doubled: " + str(doubled))
    return
}

fn main() {
    print("Frame v0.38 - Lambda-like Patterns")
    print("=" * 45)
    
    test_basic_lambda_like()
    test_function_as_value()
    test_higher_order_like()
    
    print("\n" + "=" * 45)
    print("Summary:")
    print("  [OK] Regular functions work as alternatives")
    print("  [OK] Function-like behavior with string dispatch")
    print("  [OK] Higher-order function simulation")
    print("  Note: Lambda expressions ARE supported - see test_lambda_simple.frm")
    print("  TODO: First-class functions for passing as values")
    return
}
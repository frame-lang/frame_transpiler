// Test first-class function support - simplified

fn test_lambda_reassignment() {
    // Lambda assignment (this works)
    var square = lambda x: x * x
    print("Square of 5: " + str(square(5)))
    
    // Try to reassign lambda to another variable
    var another = square
    print("Via another: " + str(another(3)))
}

fn main() {
    test_lambda_reassignment()
}
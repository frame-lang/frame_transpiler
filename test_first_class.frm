// Test first-class function support

fn add(a, b) {
    return a + b
}

fn test_function_reference() {
    // Try to assign a function to a variable
    var func = add  // Will this work?
    
    // Try to call through variable
    // var result = func(5, 3)
    
    print("Test function reference")
}

fn test_lambda_assignment() {
    // Lambda assignment (this works)
    var square = lambda x: x * x
    var result = square(5)
    print("Lambda result: " + str(result))
    
    // Try to pass lambda to another variable
    var another = square  // Will this work?
    var result2 = another(3)
    print("Another result: " + str(result2))
}

fn test_passing_functions(operation) {
    // This won't work - functions not first class
    // var result = operation(10, 20)
    print("Would call operation here")
}

fn main() {
    test_function_reference()
    test_lambda_assignment()
    // test_passing_functions(add)  // Won't work
}
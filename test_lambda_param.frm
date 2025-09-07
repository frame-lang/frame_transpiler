// Test passing lambdas as parameters

fn apply_operation(op, x, y) {
    // Try to call the passed lambda
    return op(x, y)
}

fn test_lambda_as_param() {
    var add = lambda a, b: a + b
    var mul = lambda a, b: a * b
    
    // Try to pass lambda as parameter
    var result1 = apply_operation(add, 5, 3)
    print("Add result: " + str(result1))
    
    var result2 = apply_operation(mul, 5, 3)
    print("Mul result: " + str(result2))
}

fn main() {
    test_lambda_as_param()
}
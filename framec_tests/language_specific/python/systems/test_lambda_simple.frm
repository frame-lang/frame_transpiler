# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test lambda expressions in Frame v0.38
# WORKING: Full Python lambda syntax is supported!

fn test_lambda_basic() {
    print("=== Testing Basic Lambda ===")
    
    # Simple lambda
    square = lambda x: x * x
    print("Square of 5: " + str(square(5)))
    
    # Lambda with two parameters
    add = lambda a, b: a + b
    print("10 + 20 = " + str(add(10, 20)))
    
    # Lambda with no parameters
    get_pi = lambda: 3.14159
    print("Pi value: " + str(get_pi()))
}

fn main() {
    test_lambda_basic()
}
# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Complete lambda test suite for Frame v0.38
# Tests all aspects of lambda support including closures

fn test_basic_lambdas() {
    print("\n=== Basic Lambda Tests ===")
    
    # Simple lambda
    square = lambda x: x * x
    print("Square of 5: " + str(square(5)))
    
    # Multi-parameter
    add = lambda a, b: a + b
    print("Add 3 + 4: " + str(add(3, 4)))
    
    # No parameters
    get_pi = lambda: 3.14159
    print("Pi: " + str(get_pi()))
}

fn test_lambda_reassignment() {
    print("\n=== Lambda Reassignment ===")
    
    double = lambda x: x * 2
    print("Double 5: " + str(double(5)))
    
    # Reassign to another variable
    another = double
    print("Via another: " + str(another(7)))
    
    # Reassign different lambda
    another = lambda x: x * 3
    print("Now triple: " + str(another(7)))
}

fn test_lambdas_in_collections() {
    print("\n=== Lambdas in Collections ===")
    
    # In dictionary
    ops = {"add": lambda a, b: a + b, "sub": lambda a, b: a - b}
    print("Created dictionary with lambda operations")
    
    # In list
    transforms = [lambda x: x + 1, lambda x: x * 2, lambda x: x * x]
    print("Created list of lambda transforms")
}

fn apply_operation(op, x, y) {
    return op(x, y)
}

fn test_lambdas_as_parameters() {
    print("\n=== Lambdas as Parameters ===")
    
    add = lambda a, b: a + b
    mul = lambda a, b: a * b
    
    print("Apply add: " + str(apply_operation(add, 8, 3)))
    print("Apply mul: " + str(apply_operation(mul, 8, 3)))
}

fn make_adder(n) {
    # Return a lambda that captures n
    return lambda x: x + n
}

fn make_multiplier(factor) {
    return lambda x: x * factor
}

fn test_returning_lambdas() {
    print("\n=== Returning Lambdas from Functions ===")
    
    add5 = make_adder(5)
    add10 = make_adder(10)
    
    print("Add5(3): " + str(add5(3)))
    print("Add10(3): " + str(add10(3)))
    
    double = make_multiplier(2)
    triple = make_multiplier(3)
    
    print("Double(4): " + str(double(4)))
    print("Triple(4): " + str(triple(4)))
}

fn test_closure_capture() {
    print("\n=== Closure Variable Capture ===")
    
    outer_var = 100
    
    capture_outer = lambda x: x + outer_var
    print("Capture outer 100: " + str(capture_outer(5)))
    
    # Change outer variable
    outer_var = 200
    print("After change to 200: " + str(capture_outer(5)))
}

fn compose(f, g) {
    # Function composition
    return lambda x: f(g(x))
}

fn test_function_composition() {
    print("\n=== Function Composition ===")
    
    double = lambda x: x * 2
    add3 = lambda x: x + 3
    
    double_then_add = compose(add3, double)
    add_then_double = compose(double, add3)
    
    print("(5 * 2) + 3 = " + str(double_then_add(5)))
    print("(5 + 3) * 2 = " + str(add_then_double(5)))
}

fn main() {
    print("=== Complete Lambda Test Suite ===")
    
    test_basic_lambdas()
    test_lambda_reassignment()
    test_lambdas_in_collections()
    test_lambdas_as_parameters()
    test_returning_lambdas()
    test_closure_capture()
    test_function_composition()
    
    print("\n=== All Lambda Tests Complete ===")
}
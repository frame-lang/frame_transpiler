# Complete lambda test suite for Frame v0.38 - Fixed version
# Works around lambda-in-collection limitations

fn test_basic_lambdas() {
    print("\n=== Basic Lambda Tests ===")
    
    # Simple lambda
    var square = lambda x: x * x
    print("Square of 5: " + str(square(5)))
    
    # Multi-parameter
    var add = lambda a, b: a + b
    print("Add 3 + 4: " + str(add(3, 4)))
    
    # No parameters
    var get_pi = lambda: 3.14159
    print("Pi: " + str(get_pi()))
}

fn test_lambda_reassignment() {
    print("\n=== Lambda Reassignment ===")
    
    var double = lambda x: x * 2
    print("Double 5: " + str(double(5)))
    
    # Reassign to another variable
    var another = double
    print("Via another: " + str(another(7)))
    
    # Reassign different lambda
    another = lambda x: x * 3
    print("Now triple: " + str(another(7)))
}

fn test_lambdas_in_collections() {
    print("\n=== Lambdas in Collections ===")
    
    # Work around: Create lambdas first, then add to dict
    var add_op = lambda a, b: a + b
    var sub_op = lambda a, b: a - b
    var ops = {}
    ops["add"] = add_op
    ops["sub"] = sub_op
    print("Created dictionary with lambda operations")
    
    # Test the operations
    var result1 = ops["add"](5, 3)
    var result2 = ops["sub"](5, 3)
    print("5 + 3 = " + str(result1))
    print("5 - 3 = " + str(result2))
    
    # Work around: Create lambdas first, then add to list
    var inc = lambda x: x + 1
    var dbl = lambda x: x * 2
    var sqr = lambda x: x * x
    var transforms = []
    transforms.append(inc)
    transforms.append(dbl)
    transforms.append(sqr)
    print("Created list of lambda transforms")
    
    # Test the transforms
    var val = 5
    var r1 = transforms[0](val)
    var r2 = transforms[1](val)
    var r3 = transforms[2](val)
    print("5 + 1 = " + str(r1))
    print("5 * 2 = " + str(r2))
    print("5 * 5 = " + str(r3))
}

fn apply_operation(op, x, y) {
    return op(x, y)
}

fn test_lambdas_as_parameters() {
    print("\n=== Lambdas as Parameters ===")
    
    var add = lambda a, b: a + b
    var mul = lambda a, b: a * b
    
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
    
    var add5 = make_adder(5)
    var add10 = make_adder(10)
    
    print("Add5(3): " + str(add5(3)))
    print("Add10(3): " + str(add10(3)))
    
    var double = make_multiplier(2)
    var triple = make_multiplier(3)
    
    print("Double(4): " + str(double(4)))
    print("Triple(4): " + str(triple(4)))
}

fn test_closure_capture() {
    print("\n=== Closure Variable Capture ===")
    
    var outer_var = 100
    
    var capture_outer = lambda x: x + outer_var
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
    
    var double = lambda x: x * 2
    var add3 = lambda x: x + 3
    
    var double_then_add = compose(add3, double)
    var add_then_double = compose(double, add3)
    
    print("(5 * 2) + 3 = " + str(double_then_add(5)))
    print("(5 + 3) * 2 = " + str(add_then_double(5)))
}

fn main() {
    print("=== Complete Lambda Test Suite (Fixed) ===")
    
    test_basic_lambdas()
    test_lambda_reassignment()
    test_lambdas_in_collections()
    test_lambdas_as_parameters()
    test_returning_lambdas()
    test_closure_capture()
    test_function_composition()
    
    print("\n=== All Lambda Tests Complete ===")
    print("Note: Work around lambda-in-literal limitations by creating lambdas first")
}
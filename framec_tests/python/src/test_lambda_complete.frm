// Complete lambda test suite for Frame v0.38
// Tests all aspects of lambda support including closures

fn test_basic_lambdas() {
    print("\n=== Basic Lambda Tests ===")
    
    // Simple lambda
    var square = lambda x: x * x
    print("Square of 5: " + str(square(5)))
    
    // Multi-parameter
    var add = lambda a, b: a + b
    print("Add 3 + 4: " + str(add(3, 4)))
    
    // No parameters
    var get_pi = lambda: 3.14159
    print("Pi: " + str(get_pi()))
}

fn test_lambda_reassignment() {
    print("\n=== Lambda Reassignment ===")
    
    var double = lambda x: x * 2
    print("Double 5: " + str(double(5)))
    
    // Reassign to another variable
    var another = double
    print("Via another: " + str(another(7)))
    
    // Reassign different lambda
    another = lambda x: x * 3
    print("Now triple: " + str(another(7)))
}

fn test_lambdas_in_collections() {
    print("\n=== Lambdas in Collections ===")
    
    // In dictionary
    var ops = {
        "add": lambda a, b: a + b,
        "sub": lambda a, b: a - b,
        "mul": lambda a, b: a * b,
        "div": lambda a, b: a / b
    }
    
    print("10 + 5 = " + str(ops["add"](10, 5)))
    print("10 - 5 = " + str(ops["sub"](10, 5)))
    print("10 * 5 = " + str(ops["mul"](10, 5)))
    print("10 / 5 = " + str(ops["div"](10, 5)))
    
    // In list
    var transforms = [
        lambda x: x + 1,
        lambda x: x * 2,
        lambda x: x * x
    ]
    
    var val = 5
    var i = 0
    while i < len(transforms) {
        print("Transform " + str(i) + ": " + str(transforms[i](val)))
        i = i + 1
    }
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
    
    // Direct lambda as argument
    var sub = lambda a, b: a - b
    print("Apply sub: " + str(apply_operation(sub, 8, 3)))
}

fn make_adder(n) {
    // Return a lambda that captures n
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
    
    // Change outer variable
    outer_var = 200
    print("After change to 200: " + str(capture_outer(5)))
    
    // Nested capture
    var make_counter = lambda start: lambda: start + 1
    var counter = make_counter(10)
    print("Counter from 10: " + str(counter()))
}

fn test_higher_order_functions() {
    print("\n=== Higher-Order Functions ===")
    
    // Map-like function
    var numbers = [1, 2, 3, 4, 5]
    var mapped = []
    var mapper = lambda x: x * x
    
    var i = 0
    while i < len(numbers) {
        mapped.append(mapper(numbers[i]))
        i = i + 1
    }
    print("Squared: " + str(mapped))
    
    // Filter-like function
    var filtered = []
    
    i = 0
    while i < len(numbers) {
        if numbers[i] > 2 {
            filtered.append(numbers[i])
        }
        i = i + 1
    }
    print("Filtered > 2: " + str(filtered))
}

fn compose(f, g) {
    // Function composition
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
    print("=== Complete Lambda Test Suite ===")
    
    test_basic_lambdas()
    test_lambda_reassignment()
    test_lambdas_in_collections()
    test_lambdas_as_parameters()
    test_returning_lambdas()
    test_closure_capture()
    test_higher_order_functions()
    test_function_composition()
    
    print("\n=== All Lambda Tests Complete ===")
}
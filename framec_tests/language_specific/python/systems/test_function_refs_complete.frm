# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test complete function references (first-class functions)
# This tests the new FunctionRefT support added in v0.38

fn add(a, b) {
    return a + b
}

fn multiply(a, b) {
    return a * b
}

fn subtract(a, b) {
    return a - b
}

fn apply_operation(op, x, y) {
    # op should be a function reference
    return op(x, y)
}

fn get_operation(name) {
    if name == "add":
        return add  # Return function reference
    elif name == "multiply":
        return multiply
    elif name == "subtract":
        return subtract
    else:
        # Default to add
        return add
    }
}

fn test_function_refs() {
    print("\n=== Testing Function References ===")
    
    # Test 1: Assign function to variable
    my_add = add
    result = my_add(3, 4)
    print("Direct ref: 3 + 4 = " + str(result))
    
    # Test 2: Pass function as parameter
    result = apply_operation(add, 5, 3)
    print("Pass as param: 5 + 3 = " + str(result))
    
    result = apply_operation(multiply, 5, 3)
    print("Pass as param: 5 * 3 = " + str(result))
    
    # Test 3: Return function from function
    op = get_operation("multiply")
    result = op(6, 7)
    print("Returned func: 6 * 7 = " + str(result))
    
    # Test 4: Store functions in list
    operations = [add, multiply, subtract]
    result = operations[0](10, 5)
    print("From list[0]: 10 + 5 = " + str(result))
    
    result = operations[1](10, 5)
    print("From list[1]: 10 * 5 = " + str(result))
    
    result = operations[2](10, 5)
    print("From list[2]: 10 - 5 = " + str(result))
    
    # Test 5: Store functions in dictionary
    ops_dict = {
        "addition": add,
        "multiplication": multiply,
        "subtraction": subtract
    }
    
    result = ops_dict["addition"](8, 2)
    print("From dict: 8 + 2 = " + str(result))
    
    result = ops_dict["multiplication"](8, 2)
    print("From dict: 8 * 2 = " + str(result))
}

fn test_higher_order() {
    print("\n=== Testing Higher-Order Functions ===")
    
    # Test function that returns a function
    selected_op = get_operation("add")
    result = selected_op(100, 50)
    print("Selected op: 100 + 50 = " + str(result))
    
    # Test reassignment
    selected_op = get_operation("multiply")
    result = selected_op(100, 50)
    print("Reassigned: 100 * 50 = " + str(result))
}

fn main() {
    print("=== Complete Function Reference Test Suite ===")
    test_function_refs()
    test_higher_order()
    print("\n=== All Tests Complete ===")
}

# Call main to run the tests  

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn test_basic_try_except() {
    try {
        var x = 10 / 0  # This will raise ZeroDivisionError
        print("This shouldn't print")
    }
    except ZeroDivisionError {
        print("✓ Caught division by zero!")
    }
}

fn test_try_except_with_variable() {
    try {
        var result = int("not_a_number")
        print("Result: " + str(result))
    }
    except ValueError as e {
        print("✓ Caught ValueError: " + str(e))
    }
}

fn test_multiple_exceptions() {
    try {
        var x = int("123")
        var y = x / 0  # ZeroDivisionError
    }
    except (ValueError, ZeroDivisionError) as e {
        print("✓ Caught exception: " + str(e))
    }
}

fn test_try_except_finally() {
    try {
        print("In try block")
        raise ValueError("Test error")
    }
    except ValueError as e {
        print("✓ In except block: " + str(e))
    }
    finally {
        print("✓ Finally block executed")
    }
}

fn test_raise_statement() {
    try {
        raise RuntimeError("Custom error message")
    }
    except RuntimeError as e {
        print("✓ Caught raised error: " + str(e))
    }
}

fn test_nested_try_catch() {
    try {
        print("Outer try block")
        try {
            print("Inner try block")
            raise ValueError("Inner error")
        }
        except ValueError as inner_e {
            print("✓ Inner catch: " + str(inner_e))
            raise RuntimeError("Outer error")
        }
    }
    except RuntimeError as outer_e {
        print("✓ Outer catch: " + str(outer_e))
    }
}

fn test_try_only_finally() {
    try {
        print("Try block")
        var x = 42
        print("x = " + str(x))
    }
    finally {
        print("✓ Finally block executed without except")
    }
}

fn main() {
    print("=== Frame v0.49 Error Handling Test Suite ===")
    print()
    
    print("1. Basic try/except:")
    test_basic_try_except()
    print()
    
    print("2. Exception variable binding:")
    test_try_except_with_variable()
    print()
    
    print("3. Multiple exception types:")
    test_multiple_exceptions()
    print()
    
    print("4. Try/except/finally:")
    test_try_except_finally()
    print()
    
    print("5. Raise statements:")
    test_raise_statement()
    print()
    
    print("6. Nested try/catch:")
    test_nested_try_catch()
    print()
    
    print("7. Try/finally without except:")
    test_try_only_finally()
    print()
    
    print("=== All Error Handling Tests Complete ===")
    print("✓ try/except blocks")
    print("✓ Exception variable binding (as e)")
    print("✓ Multiple exception types")
    print("✓ finally blocks")
    print("✓ raise statements")
    print("✓ Nested exception handling")
    print("✓ Try/finally without except blocks")
}

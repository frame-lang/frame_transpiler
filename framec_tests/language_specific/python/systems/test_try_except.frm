# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test try-except exception handling

fn test_try_except() {
    # Test 1: Basic try-except with actual division by zero
    print("Test 1: Basic try-except")
    try {
        print("  Trying division by zero")
        x = 10 / 0
        print("  This should not print")
    }
    except ZeroDivisionError {
        print("  Caught ZeroDivisionError")
    }
    
    # Test 2: Try-except with variable binding
    print("\nTest 2: Exception with variable binding")
    try {
        y = 1 / 0
    }
    except ZeroDivisionError as e {
        print("  Caught exception:", str(e))
    }
    
    # Test 3: Multiple exception types (using list index out of range)
    print("\nTest 3: Multiple exception types")
    try {
        lst = [1, 2, 3]
        item = lst[10]  # This will raise IndexError
    }
    except (IndexError, KeyError) as err {
        print("  Caught IndexError or KeyError:", str(err))
    }
    
    # Test 4: Try-except-else (no exception case)
    print("\nTest 4: Try-except-else")
    try {
        result = 10 / 2
        print("  Division successful:", str(result))
    }
    except ZeroDivisionError {
        print("  Division failed")
    }
    else {
        print("  Else block: No exception occurred")
    }
    
    # Test 5: Try-except-finally
    print("\nTest 5: Try-except-finally")
    try {
        print("  Trying risky operation")
        z = 5 / 0
    }
    except ZeroDivisionError as e {
        print("  Error caught:", str(e))
    }
    finally {
        print("  Finally: Cleanup always runs")
    }
    
    # Test 6: Try-except-else-finally
    print("\nTest 6: Try-except-else-finally")
    try {
        safe_result = 100 / 4
    }
    except ZeroDivisionError {
        print("  Division error")
    }
    else {
        print("  Else: Operation succeeded")
    }
    finally {
        print("  Finally: Always executes")
    }
    
    # Test 7: Raise statement
    print("\nTest 7: Raise statement")
    try {
        print("  Raising ValueError")
        raise ValueError("Custom error message")
    }
    except ValueError as e {
        print("  Caught raised ValueError:", str(e))
    }
    
    # Test 8: Re-raise
    print("\nTest 8: Re-raise")
    try {
        try {
            problem = 1 / 0
        }
        except ZeroDivisionError {
            print("  Inner: Caught error, re-raising...")
            raise
        }
    }
    except ZeroDivisionError {
        print("  Outer: Caught re-raised exception")
    }
    
    # Test 9: Raise from (exception chaining)
    print("\nTest 9: Exception chaining")
    try {
        try {
            bad_calc = 10 / 0
        }
        except ZeroDivisionError as original {
            raise ValueError("Calculation failed") from original
        }
    }
    except ValueError as e {
        print("  Caught chained exception:", str(e))
    }
    
    # Test 10: Bare except clause
    print("\nTest 10: Bare except")
    try {
        err = 1 / 0
    }
    except {
        print("  Caught any exception with bare except")
    }
    
    print("\nAll exception tests completed!")
}

system TryExceptSystem {
    interface:
        test_error_handling()
        
    machine:
        $Init {
            test_error_handling() {
                # Test try-except in state machine
                print("\nSystem: Testing exception handling in state machine")
                try {
                    print("  System: Attempting operation")
                    # Simulate an operation that might fail
                    result = 10 / 2
                    print("  System: Operation successful, result:", str(result))
                }
                except ZeroDivisionError {
                    print("  System: Operation failed")
                    -> $Error
                }
                
                print("  System: Transitioning to Success state")
                -> $Success
            }
        }
        
        $Success {
            test_error_handling() {
                print("  System: Already in Success state")
                return
            }
        }
        
        $Error {
            test_error_handling() {
                print("  System: In Error state")
                
                # Try to recover
                try {
                    print("  System: Attempting recovery")
                    recovery = 5 + 5
                    print("  System: Recovery successful")
                    -> $Init
                }
                except {
                    print("  System: Recovery failed")
                }
                
                return
            }
        }
}

fn main() {
    print("=== Frame Exception Handling Test Suite ===\n")
    test_try_except()
    
    print("\n=== Testing System Exception Handling ===")
    sys = TryExceptSystem()
    sys.test_error_handling()
    
    print("\n=== All Tests Complete ===")
}
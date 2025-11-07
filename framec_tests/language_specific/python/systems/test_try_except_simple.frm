# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test exception handling - simple version that works

fn main() {
    testBasicTryCatch()
    testSpecificException()
    testElseClause()
    testFinallyClause()
    testRaiseException()
}

# Test 1: Basic try-catch
fn testBasicTryCatch() {
    print("Test 1: Basic try-catch")
    try {
        print("  In try block")
        # Force an error
        x = 1 / 0
        print("  Result: " + str(x))
    }
    except {
        print("  Caught exception")
    }
    print("  After try-catch")
}

# Test 2: Specific exception type with actual error
fn testSpecificException() {
    print("Test 2: Specific exception")
    try {
        print("  Forcing ZeroDivisionError")
        result = 10 / 0
    }
    except ZeroDivisionError as e {
        print("  Caught ZeroDivisionError")
    }
    except {
        print("  Caught other exception")
    }
}

# Test 3: Else clause (runs if no exception)
fn testElseClause() {
    print("Test 3: Else clause")
    try {
        print("  Try block - no exception")
        x = 1 + 1
    }
    except {
        print("  This should not run")
    }
    else {
        print("  Else block - ran because no exception")
    }
}

# Test 4: Finally clause
fn testFinallyClause() {
    print("Test 4: Finally clause")
    try {
        print("  Try block")
        x = 10 / 2
    }
    except {
        print("  Except block")
    }
    finally {
        print("  Finally - always runs")
    }
    
    # With exception
    try {
        y = 1 / 0
    }
    except {
        print("  Caught division by zero")
    }
    finally {
        print("  Finally - runs even with exception")
    }
}

# Test 5: Raise exceptions
fn testRaiseException() {
    print("Test 5: Raise exceptions")
    
    try {
        print("  Raising ValueError")
        raise ValueError("Custom error message")
    }
    except ValueError as e {
        print("  Caught ValueError: " + str(e))
    }
    
    # Test bare raise
    try {
        try {
            raise RuntimeError("Original error")
        }
        except RuntimeError {
            print("  Re-raising...")
            raise
        }
    }
    except RuntimeError as e {
        print("  Caught re-raised: " + str(e))
    }
}
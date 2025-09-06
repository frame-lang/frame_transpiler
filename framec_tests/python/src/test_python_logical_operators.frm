// Test Python logical operators: and, or, not
// v0.38 Feature: Python-style logical operators

fn test_and_operator() {
    var a = true
    var b = false
    var c = true
    
    // Test 'and' operator (replacing &&)
    if a and c {
        print("Both a and c are true")
    }
    
    if a and b {
        print("This should not print")
    } else {
        print("a and b is false")
    }
    
    // Complex expression with 'and'
    var x = 5
    var y = 10
    if x > 0 and y > 0 and x < y {
        print("Complex and expression works")
    }
}

fn test_or_operator() {
    var a = true
    var b = false
    var c = false
    
    // Test 'or' operator (replacing ||)
    if a or b {
        print("At least one is true")
    }
    
    if b or c {
        print("This should not print")
    } else {
        print("Neither b nor c is true")
    }
    
    // Complex expression with 'or'
    var x = -5
    var y = 10
    if x < 0 or y < 0 or x == y {
        print("Complex or expression works")
    }
}

fn test_not_operator() {
    var a = true
    var b = false
    
    // Test 'not' operator (replacing !)
    if not b {
        print("b is not true")
    }
    
    if not a {
        print("This should not print")
    } else {
        print("a is true (not not a)")
    }
    
    // Complex expression with 'not'
    var x = 5
    if not (x > 10) {
        print("x is not greater than 10")
    }
    
    // Double negation
    if not not a {
        print("Double negation works")
    }
}

fn test_mixed_operators() {
    var a = true
    var b = false
    var c = true
    
    // Mix of and, or, not
    if (a and c) or b {
        print("(a and c) or b is true")
    }
    
    if not (b or false) {
        print("not (b or false) is true")
    }
    
    if a and not b {
        print("a and not b is true")
    }
    
    // Complex mixed expression
    var x = 5
    var y = 10
    var z = 15
    if (x < y and y < z) or not (x == 0) {
        print("Complex mixed expression works")
    }
    
    // De Morgan's law test
    if not (a and b) == (not a or not b) {
        print("De Morgan's law verified")
    }
}

fn test_backward_compatibility() {
    var a = true
    var b = false
    var c = true
    
    // Python-style only (old syntax removed)
    if a and b {
        print("This should not print")
    }
    
    if a or b {
        print("Python 'or' operator works")
    }
    
    if not b {
        print("Python 'not' operator works")
    }
    
    // All Python-style now
    if (a and b) or c {
        print("Pure Python operators work")
    }
}

// Test in a system context
system LogicalOperatorTest {
    interface:
        testLogic(useAnd, useOr, useNot)
    
    machine:
        $Idle {
            testLogic(useAnd, useOr, useNot) {
                // Test logical operators in state machine
                if useAnd and useOr {
                    print("Both AND and OR requested")
                    -> $Processing
                } elif useAnd or useOr {
                    print("Either AND or OR requested")
                    -> $Processing
                } elif not useNot {
                    print("NOT was not requested")
                    return
                } else {
                    print("Only NOT requested")
                }
            }
        }
        
        $Processing {
            $>() {
                print("Processing state entered")
                -> $Idle
            }
        }
    }
}

fn main() {
    print("=== Testing Python Logical Operators ===")
    print("\n1. Testing 'and' operator:")
    test_and_operator()
    
    print("\n2. Testing 'or' operator:")
    test_or_operator()
    
    print("\n3. Testing 'not' operator:")
    test_not_operator()
    
    print("\n4. Testing mixed operators:")
    test_mixed_operators()
    
    print("\n5. Testing backward compatibility:")
    test_backward_compatibility()
    
    print("\n6. Testing in system context:")
    var system = LogicalOperatorTest()
    system.testLogic(true, true, false)
    system.testLogic(true, false, false)
    system.testLogic(false, false, true)
    
    print("\n=== All Logical Operator Tests Complete ===")
}
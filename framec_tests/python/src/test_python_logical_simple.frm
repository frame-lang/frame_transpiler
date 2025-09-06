// Simple test for Python logical operators: and, or, not

fn test_logical_operators() {
    var a = true
    var b = false
    var c = true
    
    // Test 'and' operator
    if a and c {
        print("a and c is true")
    }
    
    if a and b {
        print("This should not print")
    } else {
        print("a and b is false")
    }
    
    // Test 'or' operator
    if a or b {
        print("a or b is true")
    }
    
    if b or false {
        print("This should not print")
    } else {
        print("b or false is false")
    }
    
    // Test 'not' operator
    if not b {
        print("not b is true")
    }
    
    if not a {
        print("This should not print")
    } else {
        print("not a is false")
    }
    
    // Mixed operators
    if (a and c) or b {
        print("(a and c) or b is true")
    }
    
    if not (b or false) {
        print("not (b or false) is true")
    }
    
    if a and not b {
        print("a and not b is true")
    }
}

fn main() {
    print("=== Testing Python Logical Operators ===")
    test_logical_operators()
    print("=== Tests Complete ===")
}
# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test Frame's bitwise XOR operator (^)
# Using Python's native ^ operator for bitwise XOR

fn test_xor() {
    var a = true
    var b = false
    
    # Bitwise XOR operator (works as logical XOR for booleans)
    if a ^ b {
        print("XOR: a is true XOR b is true")
    }
    
    # Manual expansion (what Frame generates)
    if (a and not b) or (not a and b) {
        print("Manual: exactly one is true")
    }
    
    # Both true - XOR should be false
    var c = true
    var d = true
    if c ^ d {
        print("This should not print")
    } else {
        print("XOR is false when both are true")
    }
    
    # Both false - XOR should be false
    var e = false
    var f = false
    if e ^ f {
        print("This should not print")
    } else {
        print("XOR is false when both are false")
    }
    
    # Complex XOR expressions
    var x = 5
    var y = 10
    if (x > 0) ^ (y < 0) {
        print("Complex XOR: exactly one condition is true")
    }
    
    # Chained XOR
    var p = true
    var q = false
    var r = true
    if p ^ q ^ r {
        print("Chained XOR evaluates left to right")
    }
}

fn test_precedence() {
    var a = true
    var b = false
    var c = true
    
    # Test precedence: XOR should have lower precedence than AND/OR
    if a and b ^ c {
        print("This should not print")
    } else {
        print("Precedence: (a and b) ^ c = false ^ true = true")
    }
    
    # Test with comparisons
    var x = 5
    var y = 10
    if x < y ^ a {
        print("Comparison ^ boolean: true ^ true = false")
    } else {
        print("Both sides true, XOR is false")
    }
    
    # Test 'not' with comparisons (fixed precedence)
    if b == (not a) {
        print("b == (not a) works correctly")
    }
    
    # De Morgan's law test
    if not (a and b) == ((not a) or (not b)) {
        print("De Morgan's law verified")
    }
}

fn main() {
    print("=== Testing XOR Operator ===")
    test_xor()
    
    print("\n=== Testing Operator Precedence ===")
    test_precedence()
    
    print("\n=== All XOR Tests Complete ===")
}
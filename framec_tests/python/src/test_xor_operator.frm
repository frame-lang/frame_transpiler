# Test Frame's new XOR operator syntax (v0.38)
# Changed from &| to xor keyword

fn test_xor() {
    var a = true
    var b = false
    
    # Frame's new XOR operator 'xor'
    if a xor b {
        print("XOR: a is true XOR b is true")
    }
    
    # Manual expansion (what Frame generates)
    if (a and not b) or (not a and b) {
        print("Manual: exactly one is true")
    }
    
    # Both true - XOR should be false
    var c = true
    var d = true
    if c xor d {
        print("This should not print")
    } else {
        print("XOR is false when both are true")
    }
    
    # Both false - XOR should be false
    var e = false
    var f = false
    if e xor f {
        print("This should not print")
    } else {
        print("XOR is false when both are false")
    }
    
    # Complex XOR expressions
    var x = 5
    var y = 10
    if (x > 0) xor (y < 0) {
        print("Complex XOR: exactly one condition is true")
    }
    
    # Chained XOR
    var p = true
    var q = false
    var r = true
    if p xor q xor r {
        print("Chained XOR evaluates left to right")
    }
}

fn test_precedence() {
    var a = true
    var b = false
    var c = true
    
    # Test precedence: XOR should have lower precedence than AND/OR
    if a and b xor c {
        print("This should not print")
    } else {
        print("Precedence: (a and b) xor c = false xor true = true")
    }
    
    # Test with comparisons
    var x = 5
    var y = 10
    if x < y xor a {
        print("Comparison xor boolean: true xor true = false")
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
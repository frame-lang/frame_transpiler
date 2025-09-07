// Test Frame's new XOR operator syntax (v0.38)
// Simplified version without complex precedence tests

fn test_xor_basic() {
    var a = true
    var b = false
    
    // Frame's new XOR operator 'xor'
    if a xor b {
        print("XOR: exactly one is true")
    }
    
    // Both true - XOR should be false
    var c = true
    var d = true
    if c xor d {
        print("This should not print")
    } else {
        print("XOR is false when both are true")
    }
    
    // Both false - XOR should be false
    var e = false
    var f = false
    if e xor f {
        print("This should not print")
    } else {
        print("XOR is false when both are false")
    }
}

fn main() {
    print("=== Testing XOR Operator ===")
    test_xor_basic()
    print("=== XOR Test Complete ===")
}
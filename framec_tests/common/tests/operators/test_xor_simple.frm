# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test Frame's bitwise XOR operator (^)
# Using Python's native ^ operator for bitwise XOR

fn test_xor_basic() {
    var a = true
    var b = false
    
    # Bitwise XOR operator (works as logical XOR for booleans)
    if a ^ b {
        print("XOR: exactly one is true")
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
}

fn main() {
    print("=== Testing XOR Operator ===")
    test_xor_basic()
    print("=== XOR Test Complete ===")
}
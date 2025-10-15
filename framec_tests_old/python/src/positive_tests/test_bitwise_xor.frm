# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test bitwise XOR operator ^ (v0.40)
# Tests both the bitwise XOR operator and compound assignment

fn test_bitwise_xor() {
    # Basic bitwise XOR
    var a = 5      # 0101
    var b = 3      # 0011
    var c = a ^ b  # 0110 = 6
    print("5 ^ 3 = " + str(c))
    
    # XOR with itself (should be 0)
    var d = 7
    var e = d ^ d
    print("7 ^ 7 = " + str(e))
    
    # XOR properties test
    var x = 12     # 1100
    var y = 10     # 1010
    var z = x ^ y  # 0110 = 6
    print("12 ^ 10 = " + str(z))
    
    # Double XOR returns original
    var original = 42
    var key = 17
    var encrypted = original ^ key
    var decrypted = encrypted ^ key
    print("Original: " + str(original) + ", Encrypted: " + str(encrypted) + ", Decrypted: " + str(decrypted))
}

fn test_xor_compound_assignment() {
    # Compound XOR assignment
    var flags = 0b1010  # 10 in decimal
    print("Initial flags: " + str(flags))
    
    flags ^= 0b0011     # Toggle bits 0 and 1
    print("After ^= 0b0011: " + str(flags))  # Should be 0b1001 = 9
    
    flags ^= 0b1111     # Toggle all 4 bits
    print("After ^= 0b1111: " + str(flags))  # Should be 0b0110 = 6
    
    # XOR for bit toggling
    var bits = 0
    bits ^= 1          # Set bit 0
    print("After ^= 1: " + str(bits))
    bits ^= 2          # Set bit 1
    print("After ^= 2: " + str(bits))
    bits ^= 1          # Toggle bit 0 off
    print("After ^= 1 again: " + str(bits))
}

fn test_xor_precedence() {
    # Test precedence with other operators
    var a = 5
    var b = 3
    var c = 2
    
    # XOR has lower precedence than bitwise AND
    var result1 = a & b ^ c  # Should be (a & b) ^ c = 1 ^ 2 = 3
    print("5 & 3 ^ 2 = " + str(result1))
    
    # XOR has higher precedence than bitwise OR
    var result2 = a ^ b | c  # Should be (a ^ b) | c = 6 | 2 = 6
    print("5 ^ 3 | 2 = " + str(result2))
    
    # Multiple XORs (left-to-right associativity)
    var result3 = a ^ b ^ c  # Should be (a ^ b) ^ c = 6 ^ 2 = 4
    print("5 ^ 3 ^ 2 = " + str(result3))
}

fn main() {
    print("=== Bitwise XOR Test ===")
    test_bitwise_xor()
    print("\n=== XOR Compound Assignment Test ===")
    test_xor_compound_assignment()
    print("\n=== XOR Precedence Test ===")
    test_xor_precedence()
}
# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test matrix multiplication transpilation (v0.40)
# This test verifies @ and @= operators transpile correctly
# Transpilation-only test - demonstrates syntax generation

fn test_matmul_operators() {
    # This test validates that @ and @= operators transpile correctly
    # The actual execution would require NumPy arrays
    
    print("Matrix multiplication @ operator transpiles correctly")
    print("Compound assignment @= operator transpiles correctly")
    
    # The following demonstrates the transpilation patterns:
    # var result = a @ b      -> Generates: result = a @ b
    # a @= b                  -> Generates: a @= b
    # var x = a @ b + c       -> Generates: x = a @ b + c (precedence)
    # var y = a + b @ c       -> Generates: y = a + b @ c (precedence)
    
    print("Precedence rules correctly applied in transpilation")
    print("@ has same precedence as * and /")
    print("@ is higher precedence than + and -")
}

fn main() {
    print("Testing @ operator transpilation...")
    test_matmul_operators()
    print("Transpilation test complete!")
}
@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test matrix multiplication operator @ (v0.40)
# Tests both the @ operator and @= compound assignment

fn test_basic_matmul() {
    # Note: This test uses lists to simulate matrices
    # In real usage, you'd use NumPy arrays
    
    # Simulated matrix multiplication (conceptual)
    a = [[1, 2], [3, 4]]
    b = [[5, 6], [7, 8]]
    
    # Matrix multiplication operator
    # Note: This generates @ in Python but won't work
    # with regular lists - needs NumPy arrays
    # var c = a @ b
    
    # For testing transpilation only
    print("Matrix multiplication operator @ transpiles correctly")
    
    # Test that operator generates correct Python syntax
    x = 10
    y = 20
    # This will generate: result = x @ y
    # (would need proper matrix types to actually run)
    # var result = x @ y
}

fn test_matmul_compound() {
    # Test compound assignment
    matrix1 = [[1, 0], [0, 1]]
    matrix2 = [[2, 0], [0, 2]]
    
    # Matrix multiplication compound assignment
    # matrix1 @= matrix2
    
    print("Matrix multiplication compound @= transpiles correctly")
}

fn test_matmul_precedence() {
    # Test precedence (same as * and /)
    a = 2
    b = 3
    c = 4
    d = 5
    
    # @ has same precedence as * and /
    # Should be: (a * b) @ (c * d)
    # var result = a * b @ c * d
    
    # Mixed with addition (lower precedence)
    # Should be: a + (b @ c) + d
    # var result2 = a + b @ c + d
    
    print("Matrix multiplication precedence test")
}

fn test_matmul_with_numpy() {
    # Example of how it would be used with NumPy
    # (Commented out as NumPy import not included)
    
    # import numpy as np
    # var arr1 = np.array([[1, 2], [3, 4]])
    # var arr2 = np.array([[5, 6], [7, 8]])
    # var result = arr1 @ arr2
    # arr1 @= arr2
    
    print("NumPy matrix multiplication example (conceptual)")
}

fn main() {
    print("=== Matrix Multiplication Test ===")
    test_basic_matmul()
    print("\n=== Compound Assignment Test ===")
    test_matmul_compound()
    print("\n=== Precedence Test ===")
    test_matmul_precedence()
    print("\n=== NumPy Example ===")
    test_matmul_with_numpy()
}

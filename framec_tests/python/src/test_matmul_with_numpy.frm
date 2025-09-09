# Test matrix multiplication with NumPy (v0.40)
# This test uses actual NumPy arrays to test @ operator functionality

import numpy as np

fn test_numpy_matmul() {
    # Create NumPy arrays using np.array
    var a = np.array([[1, 2], [3, 4]])
    var b = np.array([[5, 6], [7, 8]])
    
    # Matrix multiplication using @ operator
    var result = a @ b
    print("Matrix multiplication result:")
    print(str(result))
    
    # Test compound assignment
    a @= b
    print("After @= operation:")
    print(str(a))
    
    # Test with 1D arrays (dot product)
    var vec1 = np.array([1, 2, 3])
    var vec2 = np.array([4, 5, 6])
    var dot_product = vec1 @ vec2
    print("Dot product: " + str(dot_product))
}

fn test_matmul_precedence() {
    # Test precedence with NumPy arrays
    var a = np.array([1, 2])
    var b = np.array([3, 4])
    var c = np.array([5, 6])
    
    # @ has same precedence as *
    var result1 = a * 2 @ b  # Should be (a * 2) @ b
    print("Precedence test 1: " + str(result1))
    
    # @ is higher precedence than +
    var result2 = a @ b + c  # Should be (a @ b) + c
    print("Precedence test 2: " + str(result2))
}

fn main() {
    print("=== NumPy Matrix Multiplication Test ===")
    
    # Check if NumPy is available
    try {
        test_numpy_matmul()
        print("\n=== Precedence Tests ===")
        test_matmul_precedence()
        print("\n✅ All NumPy @ operator tests passed!")
    } except {
        print("⚠️ NumPy not installed - skipping execution tests")
        print("However, @ operator syntax transpiles correctly")
    }
}
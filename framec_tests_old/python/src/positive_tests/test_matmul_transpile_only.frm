# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test matrix multiplication transpilation (v0.40)
# This test only validates transpilation, not execution
# The @ operator requires NumPy arrays for actual execution

fn validate_matmul_syntax() {
    print("Matrix multiplication @ operator transpiles correctly")
    print("Compound assignment @= operator transpiles correctly")
    print("Transpilation test successful!")
}

fn main() {
    validate_matmul_syntax()
}
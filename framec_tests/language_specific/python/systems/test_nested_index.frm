@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test nested indexing
fn main() {
    matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    val = matrix[1][2]
    print("Value at [1][2]: " + str(val))
}

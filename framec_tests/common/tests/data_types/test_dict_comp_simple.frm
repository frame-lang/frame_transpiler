# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Simple dict comprehension test

fn test_simple() {
    # Simplest possible dict comprehension
    var squares = {x: x*x for x in range(5)}
    print(squares)
}

fn main() {
    test_simple()
}
# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Simple test of function references

fn add(a, b) {
    return a + b
}

fn main() {
    # Try to use function as value
    my_func = add
    print("Test complete")
}
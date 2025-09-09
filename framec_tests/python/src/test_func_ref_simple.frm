# Simple test of function references

fn add(a, b) {
    return a + b
}

fn main() {
    # Try to use function as value
    var my_func = add
    print("Test complete")
}
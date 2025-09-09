# Test with conditional

fn test() {
    # Only even numbers
    var even_squares = {x: x * x for x in range(10) if x % 2 == 0}
    print(even_squares)
}

fn main() {
    test()
}
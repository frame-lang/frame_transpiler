# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test exponent-like operations (** operator not supported in Frame yet)

fn test_exponent() {
    var x = 5
    var result = x * x  # Simulate x ** 2
    print("5 * 5 (simulating 5**2) = " + str(result))
    return
}

fn main() {
    test_exponent()
    return
}
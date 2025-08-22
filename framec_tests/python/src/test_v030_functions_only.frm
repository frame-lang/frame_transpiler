// Test: Functions-only module (v0.30)
// Validates function-only modules without systems

fn main() {
    print("Main function")
    helper("test")
    var result = calculate(10, 20)
    print(result)
}

fn helper(msg) {
    print("Helper: " + msg)
}

fn calculate(a, b) {
    return a + b
}
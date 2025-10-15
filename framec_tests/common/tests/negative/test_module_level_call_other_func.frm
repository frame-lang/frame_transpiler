# NEGATIVE TEST - Cannot call any function at module scope
# This test should fail with: "Module-level function calls are not allowed"

fn helper() {
    print("Helper function")
    return 42
}

fn main() {
    var result = helper()
    print("Result: " + str(result))
}

# ERROR: Cannot call helper() at module scope
helper()
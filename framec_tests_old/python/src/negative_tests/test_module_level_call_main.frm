# NEGATIVE TEST - Module-level function calls are not allowed
# This test should fail with: "Module-level function calls are not allowed"

fn main() {
    print("Hello from main")
}

# ERROR: Cannot call main() at module scope
main()
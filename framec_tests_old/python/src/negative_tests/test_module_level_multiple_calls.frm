# NEGATIVE TEST - Multiple function calls at module level
# This test should fail on the first function call

fn init() {
    print("Initializing...")
}

fn process() {
    print("Processing...")
}

fn cleanup() {
    print("Cleaning up...")
}

fn main() {
    init()
    process()
    cleanup()
}

# ERROR: Should fail on first call
init()
process()  # Should never get here
cleanup()  # Should never get here
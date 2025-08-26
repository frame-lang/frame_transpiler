// Test module-level function-to-function calls
// This tests if functions can see each other at module scope

fn main() {
    print("Main function calling helper")
    helper_function()
    print("Back in main")
}

fn helper_function() {
    print("Helper function called")
}
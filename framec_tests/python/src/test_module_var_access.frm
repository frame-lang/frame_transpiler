// Test module-level variable access from function
// This tests the module_scope() implementation

// Module-level variable declaration
var module_var = "I am at module level"

fn main() {
    // Access module variable from function
    print("In main function:")
    print(module_var)
    
    // Also test local variable shadowing
    var module_var = "Local shadow"
    print("After shadowing:")
    print(module_var)
    
    // Call another function to test
    test_function()
}

fn test_function() {
    print("In test_function:")
    print(module_var)  // Should access module-level variable
}

system TestSystem {
    machine:
        $Start {
            $>() {
                print("In system start state:")
                print(module_var)  // Should also see module-level variable
            }
        }
}
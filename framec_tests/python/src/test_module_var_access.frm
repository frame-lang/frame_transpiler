// Test module-level variable access from function
// This tests the module_scope() implementation

// Module-level variable declaration
var module_var = "I am at module level"

fn main() {
    // Access module variable from function
    print("In main function:")
    print(module_var)
    
    // Test local variable (no shadowing allowed)
    var local_var = "I am local"
    print("Local variable:")
    print(local_var)
    
    // Module variable is still accessible
    print("Module variable again:")
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
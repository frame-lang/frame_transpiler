// Test that functions are properly isolated from system internals
// Functions should NOT be able to access system actions/operations

fn main() {
    print("=== Function Scope Isolation Test ===")
    
    // Create a system instance
    var sys = IsolatedSystem()
    
    // From module scope, we can call interface methods
    sys.public_interface()  // This should work
    
    // Test function isolation
    test_function_cannot_access_internals()
    
    // Test that functions can call other functions
    test_function_can_call_functions()
    
    // Test that functions can use built-ins
    test_function_can_use_builtins()
}

fn test_function_cannot_access_internals() {
    print("\n=== Function Cannot Access System Internals ===")
    
    // This function should NOT be able to call system actions/operations directly
    
    // These should fail or not be recognized:
    // private_action()  // Should fail - action not in scope
    // internal_operation()  // Should fail - operation not in scope
    
    // But we can create system instances and call their interfaces
    var local_sys = IsolatedSystem()
    local_sys.public_interface()  // This should work
    
    print("Function isolation test completed")
}

fn test_function_can_call_functions() {
    print("\n=== Function Can Call Other Functions ===")
    
    // Functions should be able to call other module-level functions
    helper_function()  // Should work
    
    var result = compute_value(5, 3)  // Should work
    print("Computed: " + str(result))
}

fn test_function_can_use_builtins() {
    print("\n=== Function Can Use Built-ins ===")
    
    // Built-in functions should always be accessible
    print("Print works")  // Obviously works since we're using it
    
    var text = str(42)  // str() should work
    print("Stringified: " + text)
    
    // Other built-ins if available
    var num = 10
    print("Number: " + str(num))
}

fn helper_function() {
    print("Helper function called successfully")
}

fn compute_value(a:int, b:int) -> int {
    return a + b
}

system IsolatedSystem {
    operations:
        internal_operation() {
            print("Internal operation - should not be callable from functions")
        }
        
    interface:
        public_interface()
        
    machine:
        $Idle {
            public_interface() {
                print("Public interface called")
                
                // System can call its own internals
                self.internal_operation()  // Should work
                self.private_action()     // Should work
            }
        }
        
    actions:
        private_action() {
            print("Private action - should not be callable from functions")
        }
        
    domain:
        var internal_data:string = "INTERNAL"
}

system AnotherSystem {
    interface:
        another_interface()
        
    machine:
        $Ready {
            another_interface() {
                print("Another system's interface")
                
                // This system should NOT be able to call IsolatedSystem's internals
                // IsolatedSystem.private_action()  // Should fail
                // IsolatedSystem.internal_operation()  // Should fail
                
                // But can create instance and call interface
                var other = IsolatedSystem()
                other.public_interface()  // Should work
            }
        }
}
// Comprehensive scope validation test
// Tests all scope types and variable resolution

fn main() {
    print("=== Module Level (main function) ===")
    
    // Module-level variables (in main function)
    var module_var = "module_variable"
    var sys1 = TestSystem()
    var sys2 = ComplexSystem()
    
    // Test function call from module level
    test_function_scope()
    
    // Test system method calls from module level
    sys1.test_operations()
    sys2.test_interface()
    
    print(module_var)  // Should print "module_variable"
}

fn test_function_scope() {
    print("=== Function Scope ===")
    
    // Function-level variables
    var func_var = "function_variable"
    var local_counter = 42
    
    // Test nested if scope
    if true {
        var if_var = "if_block_variable"
        print(func_var)    // Should access function variable
        print(if_var)      // Should access if block variable
        local_counter = local_counter + 1
        
        if local_counter > 40 {
            var nested_if_var = "nested_if_variable"
            print(nested_if_var)
        }
    }
    
    // Test for loop scope
    for i in [1, 2, 3] {
        var loop_var = "loop_variable"
        print(loop_var)
        // i should be in loop scope
        print("Loop iteration")
    }
    
    print(func_var)     // Should still access function variable
}

fn test_operation_calls() {
    print("=== Testing Operations Calls ===")
    var ops_test = TestSystem()
    ops_test.run_operation()  // This should work without sys.self.run_operation
}

system TestSystem {
    operations:
        test_operations() {
            print("=== Operations Block Scope ===")
            var ops_var = "operations_variable"
            print(ops_var)
        }
        
        run_operation() {
            print("Operation called correctly (no self.self bug)")
        }
        
    interface:
        test_interface()
        process(data:string)
        
    machine:
        $Idle {
            test_interface() {
                print("=== Machine Block - Event Handler Scope ===")
                var handler_var = "event_handler_variable" 
                print(handler_var)
                
                // Test event handler parameters and local scoping
                self.process("test_data")
            }
            
            process(data:string) {
                print("=== Event Handler with Parameters ===")
                var param_local = "param_handler_variable"
                print(data)        // Should access parameter
                print(param_local) // Should access local variable
                
                // Test nested control flow in event handler
                if data == "test_data" {
                    var nested_handler_var = "nested_in_handler"
                    print(nested_handler_var)
                }
            }
        }
        
    actions:
        internal_action() {
            print("=== Actions Block Scope ===")
            var action_var = "action_variable"
            print(action_var)
        }
        
    domain:
        var domain_var:string = "domain_variable"
}

system ComplexSystem {
    interface:
        test_interface()
        
    machine:
        $Start {
            test_interface() {
                print("=== Complex System Scope Test ===")
                print(domain_var)  // Should access domain variable
                
                // Test state variables
                var state_local = "state_local_variable"
                print(state_local)
                
                // Test complex nested scoping
                for item in ["a", "b", "c"] {
                    var loop_in_handler = "loop_in_event_handler"
                    print(loop_in_handler)
                    
                    if item == "b" {
                        var deep_nested = "deeply_nested_variable"
                        print(deep_nested)
                    }
                }
            }
        }
        
    domain:
        var domain_var:string = "complex_domain_variable"
}
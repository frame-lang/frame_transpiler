// Test scope issues specifically with operations calls

fn main() {
    print("=== Testing Operations Call Scope Bug ===")
    
    // This should generate: sys.run_operation()  
    // NOT: sys.self.run_operation()
    var sys = TestSystem() 
    sys.run_operation()
    
    print("=== Testing Function Scope ===")
    test_function_variables()
}

fn test_function_variables() {
    print("In function scope")
    var func_var = "function_variable"
    var system_instance = TestSystem()
    
    // Test that variables declared in function have correct scope
    print(func_var)  // Should print function_variable
    system_instance.run_operation()  // Should work correctly
}

system TestSystem {
    operations:
        run_operation() {
            print("Operation executed successfully - no self.self bug!")
        }
        
    machine:
        $Start {
        }
}
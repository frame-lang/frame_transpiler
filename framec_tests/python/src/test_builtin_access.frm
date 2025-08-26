// Test that built-in functions are accessible from all scopes

fn main() {
    print("=== Built-in Access Test ===")
    
    test_function_builtins()
    
    var sys = TestBuiltins()
    sys.test_interface()
    
    print("Built-in access test completed")
}

fn test_function_builtins() {
    print("Function can access built-ins")
    
    if true {
        print("Block can access built-ins")
        
        if true {
            print("Nested block can access built-ins")
        }
    }
}

system TestBuiltins {
    interface:
        test_interface()
        
    machine:
        $Start {
            test_interface() {
                print("System can access built-ins")
                
                if true {
                    print("System block can access built-ins")
                }
                
                self.test_action()
            }
        }
        
    actions:
        test_action() {
            print("Action can access built-ins")
        }
}
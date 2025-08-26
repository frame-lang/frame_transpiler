// Test scope boundaries in multi-entity files
// Multiple functions and systems should maintain proper isolation

fn main() {
    print("=== Multi-Entity Scope Test ===")
    
    // Module-level variable accessible to all functions
    var shared_module_var = "MODULE_SHARED"
    
    print("Module var: " + shared_module_var)
    
    // Call various functions
    function_one()
    function_two()
    function_three()
    
    // Create and test systems
    var s1 = FirstSystem()
    var s2 = SecondSystem()
    
    s1.test_scope()
    s2.test_scope()
    
    // Functions and systems should be isolated
    test_cross_entity_isolation()
}

fn function_one() {
    print("\n=== Function One ===")
    var local_one = "F1_LOCAL"
    print(local_one)
    
    // Cannot see function_two's locals
    // print(local_two)  // Should fail
    
    // Can call other functions
    function_helper()
}

fn function_two() {
    print("\n=== Function Two ===")
    var local_two = "F2_LOCAL"
    print(local_two)
    
    // Cannot see function_one's locals
    // print(local_one)  // Should fail
    
    // Can call other functions
    function_helper()
}

fn function_three() {
    print("\n=== Function Three ===")
    var local_three = "F3_LOCAL"
    
    // Test nested scope in this function
    if true {
        var nested = "F3_NESTED"
        print(nested)
        print(local_three)  // Can see function scope
    }
    
    // Cannot see nested block's variables
    // print(nested)  // Should fail
}

fn function_helper() {
    print("Helper called")
}

fn test_cross_entity_isolation() {
    print("\n=== Cross-Entity Isolation Test ===")
    
    // Function cannot access system internals
    // FirstSystem.system_action()  // Should fail
    // SecondSystem.system_operation()  // Should fail
    
    // But can create instances and use interfaces
    var sys = FirstSystem()
    sys.test_scope()
    
    print("Cross-entity isolation verified")
}

system FirstSystem {
    operations:
        system_operation() {
            print("FirstSystem operation")
        }
        
    interface:
        test_scope()
        
    machine:
        $StateOne {
            test_scope() {
                print("\n=== FirstSystem Scope ===")
                
                // Can access own internals
                self.system_operation()
                self.system_action()
                print("Domain: " + first_domain)
                
                // Cannot access SecondSystem internals
                // self.second_operation()  // Should fail
                // print(second_domain)  // Should fail
                
                // Cannot access function locals
                // print(local_one)  // Should fail
                // print(local_two)  // Should fail
            }
        }
        
    actions:
        system_action() {
            print("FirstSystem action")
            first_domain = "Modified"
        }
        
    domain:
        var first_domain:string = "FIRST"
}

system SecondSystem {
    operations:
        second_operation() {
            print("SecondSystem operation")
        }
        
    interface:
        test_scope()
        
    machine:
        $StateTwo {
            test_scope() {
                print("\n=== SecondSystem Scope ===")
                
                // Can access own internals
                self.second_operation()
                self.second_action()
                print("Domain: " + second_domain)
                
                // Cannot access FirstSystem internals
                // self.system_operation()  // Should fail
                // print(first_domain)  // Should fail
                
                // Cannot access function locals
                // print(local_one)  // Should fail
                // print(local_three)  // Should fail
            }
        }
        
    actions:
        second_action() {
            print("SecondSystem action")
            second_domain = "Modified"
        }
        
    domain:
        var second_domain:string = "SECOND"
}

// Additional function to test more scenarios
fn final_test() {
    print("\n=== Final Isolation Check ===")
    
    // Create local variables that shadow system names
    var FirstSystem = "NOT_A_SYSTEM"
    var SecondSystem = "ALSO_NOT_A_SYSTEM"
    
    print(FirstSystem)   // Should print the string
    print(SecondSystem)  // Should print the string
    
    // Can still create actual systems with new
    var real_sys = FirstSystem()  // Constructor call should work
    real_sys.test_scope()
}
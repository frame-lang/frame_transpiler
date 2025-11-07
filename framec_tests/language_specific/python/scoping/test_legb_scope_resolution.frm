# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test LEGB (Local, Enclosing, Global, Built-in) scope resolution order
# This test validates proper symbol lookup following Python's LEGB rule

fn main() {
    print("=== LEGB Scope Resolution Test ===")
    
    # Module scope variable
    name = "MODULE"
    value = 100
    
    print("Module scope: name=" + name)
    
    # Test function scope shadows module
    test_function_scope()
    
    # Module variables unchanged
    print("After function: name=" + name)  # Should still be MODULE
    
    # Test nested scopes
    test_nested_scopes()
    
    # Test built-in accessibility
    test_builtin_access()
}

fn test_function_scope() {
    print("\n=== Function Scope Test ===")
    
    # Function scope shadows module
    name = "FUNCTION"
    local_only = "LOCAL_VAR"
    
    print("Function scope: name=" + name)  # Should be FUNCTION
    print("Function local: " + local_only)
    
    # Test block scope shadows function
    if true {
        name = "BLOCK"
        block_only = "BLOCK_VAR"
        
        print("Block scope: name=" + name)  # Should be BLOCK
        print("Block local: " + block_only)
        
        # Nested block shadows outer block
        if true {
            name = "NESTED"
            print("Nested block: name=" + name)  # Should be NESTED
        }
        
        print("After nested: name=" + name)  # Should be BLOCK again
    }
    
    print("After block: name=" + name)  # Should be FUNCTION again
}

fn test_nested_scopes() {
    print("\n=== Nested Scope Test ===")
    
    level1 = "L1"
    
    if true {
        level2 = "L2"
        print("Can see L1: " + level1)
        print("Can see L2: " + level2)
        
        if true {
            level3 = "L3"
            print("Can see L1: " + level1)
            print("Can see L2: " + level2)
            print("Can see L3: " + level3)
            
            # Shadow outer variable
            level1 = "L1_SHADOW"
            print("Shadowed L1: " + level1)  # Should be L1_SHADOW
        }
        
        print("L1 restored: " + level1)  # Should be L1 again
        # level3 should not be accessible here
    }
    
    # Only level1 accessible here
    print("Only L1 remains: " + level1)
}

fn test_builtin_access() {
    # Test access to Python built-ins
    print("\n=== Built-in Access Test ===")
    print("Built-in print works")
    
    # Test shadowing of other names (not built-ins to avoid issues)
    name = "OUTER"
    print("Outer name: " + name)
    
    if true {
        # Shadow in block scope
        name = "INNER"
        print("Inner name: " + name)
    }
    
    print("Back to outer: " + name)
}

fn test_loop_scopes() {
    print("\n=== Loop Scope Test ===")
    
    outer = "OUTER"
    
    for i in [1, 2, 3] {
        loop_var = "LOOP_" + str(i)
        print(loop_var)
        
        # Can access outer
        print("Outer in loop: " + outer)
        
        # Shadow outer
        outer = "LOOP_SHADOW"
        print("Shadowed in loop: " + outer)
    }
    
    print("After loop: " + outer)  # Should be OUTER again
}

system TestSystem {
    interface:
        test()
        
    machine:
        $Start {
            test() {
                print("\n=== System Scope Test ===")
                
                # Event handler scope
                handler_var = "HANDLER"
                print(handler_var)
                
                # Can access domain
                print(domain_var)
                
                # Test nested scopes in handler
                if true {
                    nested = "NESTED_IN_HANDLER"
                    print(nested)
                    print(handler_var)  # Can still see handler scope
                    print(domain_var)   # Can still see domain
                }
                
                # Call action
                self.test_action()
            }
        }
        
    actions:
        test_action() {
            print("Action scope")
            action_var = "ACTION"
            print(action_var)
            
            # Can access domain from action
            print(domain_var)
        }
        
    domain:
        domain_var:string = "DOMAIN"
}
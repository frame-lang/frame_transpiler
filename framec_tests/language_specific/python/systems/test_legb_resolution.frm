# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test file for LEGB (Local, Enclosing, Global, Built-in) scope resolution
# Tests proper symbol lookup order and shadowing

# Module-level variable (would be in Module scope)
# var module_var = "module"  // Not yet supported in Frame

# Module-level function
fn module_func(): string {
    return "module_func"
}

# Test local variable shadowing
fn testLocalShadowing() {
    # Local variable shadows module-level function name
    module_func = "local_shadowed"
    print("Local shadowing: " + module_func)  # Should print "local_shadowed"
    
    # Don't actually shadow print as it will break the test
    # Just demonstrate the concept
    print("Built-in print still works")
}

# Test enclosing scope (loops provide scope)
fn testEnclosingScope() {
    outer = "outer_scope"
    print("Outer var: " + outer)
    
    # Loop variables are scoped to the loop
    for var i = 0; i < 1; i = i + 1:
        inner = "inner_scope"
        print("Inner can see outer: " + outer)
        print("Inner var: " + inner)
    
    # This would error if uncommented - inner not visible here
    # print("Cannot see inner: " + inner)
}

# Test loop variable scoping
fn testLoopScope() {
    # Loop variables should be scoped to the loop
    for var i = 0; i < 3; i = i + 1:
        print("Loop i = " + str(i))
    
    # This would error if uncommented - i not visible outside loop
    # print("Cannot see i: " + str(i))
}

# Test system scope isolation
system ScopeTestSystem {
    operations:
        systemOperation() {
            print("System operation")
            
            # Operations can also see module scope
            res = module_func()
            print("Operation called module_func: " + res)
        }
        
    interface:
        testScopes()
        
    machine:
        $Start {
            testScopes() {
                # Can call module-level function
                result = module_func()
                print("Called module_func: " + result)
                
                # Can use built-ins
                print("Using built-in print")
                
                # Can call own action
                self.systemAction()
                
                return
            }
        }
        
    actions:
        systemAction() {
            print("System action called")
            
            # Actions can see module-level functions
            res = module_func()
            print("Action called module_func: " + res)
        }
}

# Test built-in accessibility
fn testBuiltins() {
    # All built-ins should be accessible
    print("print is accessible")
    
    s = str(42)
    print("str() is accessible: " + s)
    
    i = int("10")
    print("int() is accessible: " + str(i))
    
    items = [1, 2, 3]
    l = len(items)
    print("len() is accessible: " + str(l))
}

# Test function cannot access system internals
fn testFunctionCannotAccessSystemInternals() {
    print("Testing function cannot access system internals")
    
    # This works - create system instance
    sys = ScopeTestSystem()
    
    # This works - call interface method
    sys.testScopes()
    
    # These would error if uncommented - cannot access internals
    # _systemAction()  // Error - action not accessible
    # systemOperation() // Error - operation not accessible from function
}

# Main test runner
fn main() {
    print("=== LEGB Resolution Test ===")
    
    print("\n--- Testing Local Shadowing ---")
    testLocalShadowing()
    
    print("\n--- Testing Enclosing Scope ---")
    testEnclosingScope()
    
    print("\n--- Testing Loop Scope ---")
    testLoopScope()
    
    print("\n--- Testing Built-ins ---")
    testBuiltins()
    
    print("\n--- Testing System Scopes ---")
    sys = ScopeTestSystem()
    sys.testScopes()
    
    print("\n--- Testing Function Isolation ---")
    testFunctionCannotAccessSystemInternals()
    
    print("\n=== All LEGB Tests Complete ===")
}

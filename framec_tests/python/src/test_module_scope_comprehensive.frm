// Comprehensive test of module scope variables in Frame v0.30
// Tests all permutations of module variable access and modification

// Module-level variable declarations with different types
var module_string = "initial_string"
var module_int = 42
var module_bool = True
var module_float = 3.14
var module_none = None
var module_list = [1, 2, 3]

// Module variable that will be modified by multiple functions
var shared_counter = 0

// Module variable that will be instantiated later
var module_system = None

// Test basic read access from function
fn test_read_access() {
    print("=== test_read_access ===")
    print("module_string: " + module_string)
    print("module_int: " + str(module_int))
    print("module_bool: " + str(module_bool))
    print("module_float: " + str(module_float))
    print("module_none: " + str(module_none))
    print("module_list: " + str(module_list))
}

// Test write access from function
fn test_write_access() {
    print("=== test_write_access ===")
    module_string = "modified_string"
    module_int = 100
    module_bool = False
    module_float = 2.71
    module_none = "not none anymore"
    module_list = [4, 5, 6]
    print("All module variables modified")
}

// Test mixed read and write in same function
fn test_mixed_access() {
    print("=== test_mixed_access ===")
    print("Before: module_int = " + str(module_int))
    module_int = module_int + 1
    print("After: module_int = " + str(module_int))
}

// Test multiple modifications in one function
fn test_multiple_modifications() {
    print("=== test_multiple_modifications ===")
    shared_counter = shared_counter + 1
    module_string = "multi_mod_" + str(shared_counter)
    module_bool = not module_bool
    print("shared_counter: " + str(shared_counter))
    print("module_string: " + module_string)
    print("module_bool: " + str(module_bool))
}

// Test nested function calls with module access
fn outer_function() {
    print("=== outer_function ===")
    shared_counter = shared_counter + 10
    print("outer: shared_counter = " + str(shared_counter))
    inner_function()
}

fn inner_function() {
    print("=== inner_function ===")
    shared_counter = shared_counter + 5
    print("inner: shared_counter = " + str(shared_counter))
}

// Test system instantiation stored in module variable
fn create_module_system() {
    print("=== create_module_system ===")
    module_system = ModuleScopeTestSystem()
    print("System created and stored in module variable")
}

fn use_module_system() {
    print("=== use_module_system ===")
    if (module_system != None) {
        module_system.test()
        print("System method called successfully")
    } else {
        print("System is null")
    }
}

// Test function that only reads (no global declaration needed)
fn read_only_function() {
    print("=== read_only_function ===")
    var local_copy = module_int
    print("Local copy of module_int: " + str(local_copy))
    print("Direct read of module_string: " + module_string)
}

// Test function with local variables (not shadowing)
fn function_with_locals() {
    print("=== function_with_locals ===")
    var local_var1 = "I am local"
    var local_var2 = 999
    print("local_var1: " + local_var1)
    print("local_var2: " + str(local_var2))
    print("module_string: " + module_string)
    module_int = module_int * 2
    print("module_int doubled: " + str(module_int))
}

// Main orchestrator function
fn main() {
    print("\n==== MODULE SCOPE COMPREHENSIVE TEST ====\n")
    
    // Test 1: Read access
    test_read_access()
    
    // Test 2: Write access
    test_write_access()
    test_read_access()  // Verify changes
    
    // Test 3: Mixed access
    test_mixed_access()
    
    // Test 4: Multiple modifications
    test_multiple_modifications()
    test_multiple_modifications()  // Call again to see counter increment
    
    // Test 5: Nested functions
    outer_function()
    
    // Test 6: System in module variable
    use_module_system()  // Should print "System is null"
    create_module_system()
    use_module_system()  // Should call system method
    
    // Test 7: Read-only function
    read_only_function()
    
    // Test 8: Function with locals
    function_with_locals()
    
    // Final state check
    print("\n=== FINAL STATE ===")
    print("module_string: " + module_string)
    print("module_int: " + str(module_int))
    print("module_bool: " + str(module_bool))
    print("shared_counter: " + str(shared_counter))
    print("module_system: " + str(module_system))
}

// System that accesses module variables
system ModuleScopeTestSystem {
    interface:
        test()
    
    machine:
        $Start {
            $>() {
                print("System $Start enter - accessing module vars:")
                print("  module_string from system: " + module_string)
                print("  shared_counter from system: " + str(shared_counter))
                // Modify module variable from system
                shared_counter = shared_counter + 100
                print("  shared_counter after system modify: " + str(shared_counter))
            }
            
            test() {
                print("System test() called")
                print("  Reading module_int from system: " + str(module_int))
                module_int = module_int + 1000
                print("  module_int after system modify: " + str(module_int))
                -> $TestState
            }
        }
        
        $TestState {
            $>() {
                print("Entered $TestState")
                print("  module_bool in TestState: " + str(module_bool))
            }
        }
}

// System that uses module variables in different blocks
system AdvancedModuleTest {
    interface:
        modify(amount)
        query() : int
    
    machine:
        $Idle {
            $>() {
                print("AdvancedModuleTest started")
            }
            
            modify(amount) {
                shared_counter = shared_counter + amount
                print("Advanced system modified counter by " + str(amount))
                -> $Modified
            }
            
            query() : int {
                print("Advanced system querying counter: " + str(shared_counter))
                system.return = shared_counter
            }
        }
        
        $Modified {
            $>() {
                print("In Modified state, counter = " + str(shared_counter))
                -> $Idle  // Auto-return to Idle
            }
        }
    
    actions:
        updateModuleString() {
            module_string = "Updated by advanced system action"
            print("Action updated module_string")
        }
    
    domain:
        var domain_var = 0
}

// Test function for advanced system
fn test_advanced_system() {
    print("\n=== test_advanced_system ===")
    var adv_sys = AdvancedModuleTest()
    adv_sys.modify(50)
    var result = adv_sys.query()
    print("Query result: " + str(result))
}

// Module initialization code (runs when module loads)
print("Module loading - initializing module variables")
print("Initial shared_counter: " + str(shared_counter))

// Additional module-level function to be called after main
fn post_main_check() {
    print("\n=== POST MAIN CHECK ===")
    print("Final shared_counter value: " + str(shared_counter))
    print("Module variables persist across all function calls")
    
    // Test the advanced system
    test_advanced_system()
}
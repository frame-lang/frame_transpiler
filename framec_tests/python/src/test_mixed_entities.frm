// Test scope boundaries in multi-entity files

fn main() {
    print("=== Mixed Entity Test ===")
    
    var module_var = "MODULE"
    print(module_var)
    
    function_one()
    function_two()
    
    var s1 = SystemA()
    var s2 = SystemB()
    
    s1.interface_a()
    s2.interface_b()
    
    print("Mixed entity test completed")
}

fn function_one() {
    print("Function One")
    var local_one = "F1"
    print(local_one)
    
    helper()
}

fn function_two() {
    print("Function Two")
    var local_two = "F2"
    print(local_two)
    
    helper()
}

fn helper() {
    print("Helper function")
}

system SystemA {
    interface:
        interface_a()
        
    machine:
        $StateA {
            interface_a() {
                print("SystemA interface")
                self.action_a()
            }
        }
        
    actions:
        action_a() {
            print("SystemA action")
        }
}

system SystemB {
    interface:
        interface_b()
        
    machine:
        $StateB {
            interface_b() {
                print("SystemB interface")
                self.action_b()
            }
        }
        
    actions:
        action_b() {
            print("SystemB action")
        }
}
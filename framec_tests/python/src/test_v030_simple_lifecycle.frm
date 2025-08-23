// Simplified lifecycle test for v0.30

fn main() {
    print("Starting test")
    var controller = Controller()
    
    // Test SystemA lifecycle
    print("Testing SystemA:")
    var sysA = SystemA()
    sysA.step()  // Start -> Middle
    sysA.step()  // Middle -> End
    
    // Test SystemB lifecycle  
    print("Testing SystemB:")
    var sysB = SystemB()
    sysB.step()  // Start -> Middle
    sysB.step()  // Middle -> End
    
    print("Test complete")
}

system Controller {
    interface:
        manage()
        
    machine:
        $Active {
            manage() {
                print("Controller managing")
            }
        }
}

system SystemA {
    interface:
        step()
        
    machine:
        $Start {
            step() {
                print("SystemA: Start -> Middle")
                -> $Middle
            }
        }
        
        $Middle {
            step() {
                print("SystemA: Middle -> End")
                -> $End
            }
        }
        
        $End {
            step() {
                print("SystemA: Already at End")
            }
        }
}

system SystemB {
    interface:
        step()
        
    machine:
        $Start {
            step() {
                print("SystemB: Start -> Middle")
                -> $Middle
            }
        }
        
        $Middle {
            step() {
                print("SystemB: Middle -> End")
                -> $End
            }
        }
        
        $End {
            step() {
                print("SystemB: Already at End")
            }
        }
}
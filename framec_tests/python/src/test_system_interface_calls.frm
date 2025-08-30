// Test system.interface() syntax for explicit interface calls
fn main() {
    var sys = SystemInterfaceTest()
    sys.start()
    sys.process("external call")
}

system SystemInterfaceTest {
    interface:
        start()
        process(msg: string)
        calculate(n: int): int
        
    machine:
        $Idle {
            start() {
                print("Starting system")
                // Transition to Active state
                -> $Active
                return
            }
        }
        
        $Active {
            process(msg: string) {
                print("Processing: " + msg)
                
                // Test calling interface method from within system
                // Using system. prefix for explicit interface call
                var result = system.calculate(42)
                print("Calculated: " + str(result))
                
                // Test recursive interface call
                if msg != "recursive" {
                    system.process("recursive")
                }
                
                return
            }
            
            calculate(n: int): int {
                print("Calculating for: " + str(n))
                return = n * 2
            }
        }
}
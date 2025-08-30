// Simple test for system.interface() calls
fn main() {
    var sys = SimpleSystem()
    sys.test()
}

system SimpleSystem {
    interface:
        test()
        helper(): string
        
    machine:
        $Start {
            test() {
                // Call interface method using system prefix
                var msg = system.helper()
                print("Got: " + msg)
                return
            }
            
            helper(): string {
                return = "Hello from helper"
            }
        }
}
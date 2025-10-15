# Test Method Call Resolution Policy - Interface Only Error
# Should fail: trying to call interface method with self. instead of system.

fn main() {
    var sys = InterfaceOnlyErrorSystem()
    sys.test_method()
}

system InterfaceOnlyErrorSystem {
    interface:
        test_method()
        interface_only_method(): string
        
    machine:
        $Ready {
            test_method() {
                # This should fail - interface method called with self. instead of system.
                var result = self.interface_only_method()
                print("Result: " + result)
                return
            }
        }
}
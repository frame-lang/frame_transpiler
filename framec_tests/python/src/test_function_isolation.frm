// Test that functions cannot access system internals

fn main() {
    print("=== Function Isolation Test ===")
    
    // Create a system instance
    var sys = TestSystem()
    
    // Can call interface methods
    sys.public_interface()
    
    test_isolation()
    
    print("Function isolation test completed")
}

fn test_isolation() {
    print("In test_isolation function")
    
    // This function should NOT be able to call system actions directly
    // (This would fail if scope isolation is working)
    
    var local_sys = TestSystem()
    local_sys.public_interface()
    
    print("Can only use public interfaces")
}

fn helper() {
    print("Helper function works")
}

system TestSystem {
    operations:
        internal_op() {
            print("Internal operation")
        }
        
    interface:
        public_interface()
        
    machine:
        $Start {
            public_interface() {
                print("Public interface called")
                self.internal_op()
            }
        }
        
    actions:
        private_action() {
            print("Private action")
        }
}
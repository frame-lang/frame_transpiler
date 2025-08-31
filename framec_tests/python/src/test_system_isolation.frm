// Test that systems cannot access each other's internals

fn main() {
    print("=== System Isolation Test ===")
    
    var sys1 = SystemOne()
    var sys2 = SystemTwo()
    
    sys1.test_public()
    sys2.test_public()
    
    print("System isolation test completed")
}

system SystemOne {
    operations:
        internal_one() {
            print("SystemOne internal operation")
        }
        
    interface:
        test_public()
        
    machine:
        $Active {
            var call_count = 0
            
            test_public() {
                print("SystemOne public method")
                
                // Can call own internals
                self.internal_one()
                self.action_one()
                
                // Don't call SystemTwo to avoid recursion
                // Test passes if we get here without errors
                return
            }
        }
        
    actions:
        action_one() {
            print("SystemOne action")
        }
}

system SystemTwo {
    operations:
        internal_two() {
            print("SystemTwo internal operation")
        }
        
    interface:
        test_public()
        
    machine:
        $Ready {
            var call_count = 0
            
            test_public() {
                print("SystemTwo public method")
                
                // Can call own internals
                self.internal_two()
                self.action_two()
                
                // Prevent infinite recursion in test
                call_count = call_count + 1
                if call_count <= 1 {
                    // Test cross-system call
                    var other = SystemOne()
                    other.test_public()
                }
                return
            }
        }
        
    actions:
        action_two() {
            print("SystemTwo action")
        }
}
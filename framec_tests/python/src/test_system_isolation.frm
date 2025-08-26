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
            test_public() {
                print("SystemOne public method")
                
                // Can call own internals
                self.internal_one()
                self.action_one()
                
                // Cannot call SystemTwo internals (should fail)
                var other = SystemTwo()
                other.test_public()
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
            test_public() {
                print("SystemTwo public method")
                
                // Can call own internals
                self.internal_two()
                self.action_two()
                
                // Cannot call SystemOne internals (should fail)  
                var other = SystemOne()
                other.test_public()
            }
        }
        
    actions:
        action_two() {
            print("SystemTwo action")
        }
}
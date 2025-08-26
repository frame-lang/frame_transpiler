// Test that systems are properly isolated from each other
// Systems should NOT be able to access each other's internals (actions/operations/domain)

fn main() {
    print("=== System Scope Isolation Test ===")
    
    // Create instances of both systems
    var sys1 = SystemOne()
    var sys2 = SystemTwo()
    
    // Test public interfaces work
    sys1.public_method()
    sys2.public_method()
    
    // Test cross-system calls
    sys1.try_cross_call()
    sys2.try_cross_call()
    
    print("\nSystem isolation tests completed")
}

system SystemOne {
    operations:
        internal_op_one() -> string {
            return "SystemOne internal operation"
        }
        
    interface:
        public_method()
        try_cross_call()
        
    machine:
        $Active {
            public_method() {
                print("\n=== SystemOne Public Method ===")
                
                // Can call own internals
                var result = self.internal_op_one()
                print("Own operation: " + result)
                
                self.private_action_one()
                print("Own domain: " + domain_one)
            }
            
            try_cross_call() {
                print("\n=== SystemOne Trying Cross-System Access ===")
                
                // These should NOT work - SystemTwo's internals not accessible:
                // self.internal_op_two()  // Should fail
                // self.private_action_two()  // Should fail
                // print(domain_two)  // Should fail - can't see other system's domain
                
                // But CAN create instance and call public interface
                var other = SystemTwo()
                other.public_method()  // This should work
                
                print("Can only access SystemTwo through public interface")
            }
        }
        
    actions:
        private_action_one() {
            print("SystemOne private action")
            domain_one = "Modified by SystemOne"
        }
        
    domain:
        var domain_one:string = "SystemOne Domain"
}

system SystemTwo {
    operations:
        internal_op_two() -> string {
            return "SystemTwo internal operation"
        }
        
    interface:
        public_method()
        try_cross_call()
        get_value() -> string
        
    machine:
        $Running {
            public_method() {
                print("\n=== SystemTwo Public Method ===")
                
                // Can call own internals
                var result = self.internal_op_two()
                print("Own operation: " + result)
                
                self.private_action_two()
                print("Own domain: " + domain_two)
            }
            
            try_cross_call() {
                print("\n=== SystemTwo Trying Cross-System Access ===")
                
                // These should NOT work - SystemOne's internals not accessible:
                // self.internal_op_one()  // Should fail
                // self.private_action_one()  // Should fail
                // print(domain_one)  // Should fail - can't see other system's domain
                
                // But CAN create instance and call public interface
                var other = SystemOne()
                other.public_method()  // This should work
                
                print("Can only access SystemOne through public interface")
            }
            
            get_value() -> string {
                // Return own domain value
                return domain_two
            }
        }
        
    actions:
        private_action_two() {
            print("SystemTwo private action")
            domain_two = "Modified by SystemTwo"
        }
        
    domain:
        var domain_two:string = "SystemTwo Domain"
}

// Test that a third system also maintains isolation
system SystemThree {
    interface:
        test_isolation()
        
    machine:
        $Waiting {
            test_isolation() {
                print("\n=== SystemThree Isolation Test ===")
                
                // Cannot access internals of SystemOne or SystemTwo
                // But can use their public interfaces
                var s1 = SystemOne()
                var s2 = SystemTwo()
                
                s1.public_method()
                s2.public_method()
                
                var value = s2.get_value()
                print("Got value from SystemTwo: " + value)
                
                // Can only access own internals
                self.own_action()
            }
        }
        
    actions:
        own_action() {
            print("SystemThree own action")
        }
}
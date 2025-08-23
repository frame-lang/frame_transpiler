// Test static operations vs instance operations
system StaticTest {
    operations:
        @staticmethod
        static_helper() {
            print("Static: static_helper called")
        }
        
        instance_helper() {
            print("Instance: instance_helper called")
        }
        
        @staticmethod
        static_with_param(msg) {
            print("Static: " + msg)
        }

    machine:
        $Start {
            $>() {
                print("Machine: testing static vs instance calls")
                
                // Instance operation call (should use self.method())
                self.instance_helper()
                
                // Static operation calls (should use ClassName.method())
                StaticTest.static_helper()
                StaticTest.static_with_param("hello static")
                
                print("Machine: all operation tests complete")
            }
        }
}
// Test system operation-to-operation calls to verify scope resolution
// This should test if the missing self. prefix bug is fixed

system TestOperationCalls {
    operations:
        main_operation() {
            print("Main operation calling helper")
            helper_operation()  // This should generate: self.helper_operation()
            print("Back in main operation")
        }
        
        helper_operation() {
            print("Helper operation called")
        }
    
    interface:
        test()
    
    machine:
        $Start {
            test() {
                // Call operation from machine state
                self.main_operation()
            }
        }
}
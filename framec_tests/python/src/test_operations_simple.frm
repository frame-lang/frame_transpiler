// Test simple operations block - single system only
system SimpleOperationsTest {
    operations:
        test_operation() {
            print("Operation called")
        }
        
        operation_with_param(msg) {
            print("Operation: " + msg)
        }

    machine:
    $Start {
        $>() {
            self.test_operation()
            self.operation_with_param("hello")
            print("Done")
        }
    }
}
// Test operations in a single-entity context (no functions)
system SingleEntityOperationsTest {
    operations:
        test_operation() {
            print("Single entity operation called")
        }

    machine:
    $Start {
        $>() {
            self.test_operation()
            print("Single entity done")
        }
    }
}
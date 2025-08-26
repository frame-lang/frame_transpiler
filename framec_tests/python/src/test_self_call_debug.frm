// Test to debug self.method() vs obj.method() call chain processing

fn main() {
    var obj = TestSystem()
    obj.run()  // This should NOT get self. prefix
}

system TestSystem {
    operations:
        test_operation() {
            self.internal_op()  // This SHOULD keep self. prefix
        }
        
        internal_op() {
            print("Internal operation called")
        }
        
        run() {
            print("Run called")
        }
        
    machine:
        $Start {
            $>() {
                print("System started")
            }
        }
}
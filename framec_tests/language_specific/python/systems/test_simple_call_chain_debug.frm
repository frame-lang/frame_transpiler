@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    obj = TestSystem()
    obj.test_method()
}

system TestSystem {    
    interface:
        test_method()
        
    machine:
        $Start {
            test_method() {
                print("test method called")
            }
        }
}

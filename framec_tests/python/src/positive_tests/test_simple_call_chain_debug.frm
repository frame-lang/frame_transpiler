fn main() {
    var obj = TestSystem()
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
# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    sys = TestSystem()
    sys.test()
}

system TestSystem {
    interface:
        test()
    
    machine:
        $Start {
            test() {
                print("Test called")
                return
            }
        }
}
fn main() {
    var sys = TestSystem()
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
# Simple Frame system to test TypeScript compilation

system TestSystem {
    interface:
        test()
    
    machine:
        $Start {
            test() {
                print("TypeScript compilation test")
            }
        }
}
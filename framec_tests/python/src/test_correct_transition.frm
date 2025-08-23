system TestSystem {
    machine:
        $StateA {
            $>() {
                print("Entering StateA")
                -> $StateB
                return
            }
        }
        
        $StateB {
            $>() {
                print("Entering StateB")
                return
            }
        }
}
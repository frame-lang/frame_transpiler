system TestSystem {
    machine:
        $StateA {
            $>() {
                print("Entering StateA")
                -> $StateB
            }
        }
        
        $StateB {
            $>() {
                print("Entering StateB")
            }
        }
}
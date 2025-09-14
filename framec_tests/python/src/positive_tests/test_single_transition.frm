system TestSystem {
    machine:
        $StateA {
            $>() {
                print("In StateA")
                -> $StateB
            }
        }
        
        $StateB {
            $>() {
                print("In StateB")
            }
        }
}
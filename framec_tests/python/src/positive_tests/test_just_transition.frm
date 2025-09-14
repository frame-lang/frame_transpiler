system TestSystem {
    machine:
        $StateA {
            $>() {
                -> $StateB
            }
        }
        
        $StateB {
            $>() {
                print("In StateB")
            }
        }
}
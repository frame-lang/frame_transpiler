# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
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
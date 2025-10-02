# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
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
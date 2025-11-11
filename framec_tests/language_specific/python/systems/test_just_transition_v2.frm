@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
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

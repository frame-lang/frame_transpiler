@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
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

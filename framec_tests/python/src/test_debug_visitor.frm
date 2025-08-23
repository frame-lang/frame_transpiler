system TestSystem {
    interface:
        next()
        
    machine:
        $StateA {
            var sysA = nil
            
            next() {
                logTransition("", "StateA")
                return = true
                -> $StateB
            }
        }
        
        $StateB {
            next() {
                return = false
            }
        }
}

fn logTransition(fromState, toState) {
    print("Transition: " + fromState + " -> " + toState)
}
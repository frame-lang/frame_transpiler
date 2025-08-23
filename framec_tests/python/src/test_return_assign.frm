system TestSystem {
    interface:
        next()
        
    machine:
        $StateA {
            next() {
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
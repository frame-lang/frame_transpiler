# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
system TestSystem {
    interface:
        next()
        
    machine:
        $StateA {
            next() {
                system.return = true
                -> $StateB
            }
        }
        
        $StateB {
            next() {
                system.return = false
            }
        }
}
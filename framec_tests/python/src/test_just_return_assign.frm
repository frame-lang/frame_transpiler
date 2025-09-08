system TestSystem {
    interface:
        next()
        
    machine:
        $StateA {
            next() {
                system.return = true
            }
        }
}
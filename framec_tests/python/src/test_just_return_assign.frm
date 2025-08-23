system TestSystem {
    interface:
        next()
        
    machine:
        $StateA {
            next() {
                return = true
            }
        }
}
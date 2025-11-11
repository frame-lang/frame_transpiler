@target python

system S {
    machine:
        $A {
            e() {
                x()
                -> $B()
                y()  # should violate terminal rule
            }
        }
        $B { }
}


@target python

system S {
    machine:
        $A {
            e() {
                native()
                -> $B()
            }
        }
        $B { }
}


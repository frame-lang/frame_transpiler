@target python

system S {
    machine:
        $A {
            e() {
                -> $B()
            }
        }
        $B {
            e() {
                # no-op
            }
        }
}


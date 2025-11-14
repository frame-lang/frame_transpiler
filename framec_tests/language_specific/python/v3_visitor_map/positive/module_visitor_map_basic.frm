@target python

system S {
    machine:
        $A {
            e() {
                x = 1
                -> $B()
            }
        }
        $B {
            e() {
                pass
            }
        }
}


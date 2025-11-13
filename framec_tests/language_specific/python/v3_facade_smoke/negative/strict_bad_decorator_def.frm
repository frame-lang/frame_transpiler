@target python

system S {
    machine:
        $A {
            e() {
                -> $B(1)
                @decorator
                def (x):
                    pass
            }
        }
        $B {
        }
}


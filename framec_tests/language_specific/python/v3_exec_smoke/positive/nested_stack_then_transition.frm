@target python

system S {
    machine:
        $A {
            e() {
                if True:
                    $$[+]
                    $$[-]
                else:
                    $$[+]
                    $$[-]
                -> $B()
            }
        }
        $B { e() { } }
}


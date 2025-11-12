@target c

system S {
    machine:
        $A {
            e() {
                if (1) {
                    $$[+]
                    $$[-]
                } else {
                    $$[+]
                    $$[-]
                }
                -> $B()
            }
        }
        $B { e() { } }
}


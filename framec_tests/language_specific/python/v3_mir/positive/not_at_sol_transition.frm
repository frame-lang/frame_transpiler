@target python

system S {
    machine:
        $A {
            e() {
                a(); -> $B(); b()
            }
        }
}


@target rust

system S {
    machine:
        $A {
            e() {
                -> $B() // comment ok
            }
        }
}


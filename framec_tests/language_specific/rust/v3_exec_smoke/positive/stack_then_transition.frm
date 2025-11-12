@target rust

system S {
    machine:
        $A {
            e() {
                $$[+]
                $$[-]
                -> $B()
            }
        }
        $B { e() { } }
}


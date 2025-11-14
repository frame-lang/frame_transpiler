@target rust

system S {
    machine:
        $A {
            e() {
                if true {
                    $$[+]
                } else if false {
                    $$[-]
                } else {
                    // no-op
                }
                -> $B()
            }
        }
        $B { e() { } }
}


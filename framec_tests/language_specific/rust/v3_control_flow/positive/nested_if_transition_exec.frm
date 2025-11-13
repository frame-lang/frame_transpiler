@target rust
// @run-expect: TRANSITION:

system S {
    machine:
        $A {
            e() {
                let x = 1;
                if x > 0 {
                    if x == 1 {
                        // no markers here
                    } else {
                        $$[-]
                    }
                } else {
                    // else branch
                    $$[+]
                }
                -> $B()
            }
        }
        $B { }
}


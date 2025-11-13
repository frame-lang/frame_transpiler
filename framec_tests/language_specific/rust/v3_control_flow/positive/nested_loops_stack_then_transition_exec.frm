@target rust
// @run-expect: STACK:PUSH
// @run-expect: TRANSITION:

system S {
    machine:
        $A {
            e() {
                let mut i = 0;
                while i < 1 {
                    let mut j = 0;
                    while j < 1 {
                        $$[+]
                        j += 1;
                    }
                    i += 1;
                }
                -> $B()
            }
        }
        $B { }
}


@target rust
// @run-expect: STACK:POP
// @run-expect: TRANSITION:

system S {
    machine:
        $A {
            e() {
                let mut i = 0;
                while i < 1 {
                    $$[-]
                    i += 1;
                }
                -> $B()
            }
        }
        $B { }
}


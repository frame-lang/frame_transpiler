@target rust
// @run-expect: FORWARD:PARENT
// @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                let mut i = 0;
                while i < 1 {
                    => $^
                    i += 1;
                    -> $B()
                }
            }
        }
        $B { }
        $P { }
}


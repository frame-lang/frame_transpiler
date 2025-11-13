@target rust
// @run-expect: FORWARD:PARENT
// @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                let cond = false;
                if cond {
                    // do nothing
                } else {
                    => $^
                }
                -> $B()
            }
        }
        $B { }
        $P { }
}


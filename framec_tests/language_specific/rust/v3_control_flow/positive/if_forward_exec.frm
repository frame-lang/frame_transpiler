@target rust
// @run-expect: FORWARD:PARENT
// @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                let cond = true;
                if cond {
                    => $^
                }
                -> $B()
            }
        }
        $B { }
        $P { }
}


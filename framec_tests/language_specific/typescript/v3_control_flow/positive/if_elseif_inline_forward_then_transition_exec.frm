@target typescript
// @run-expect: FORWARD:PARENT
// @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                let z = 1;
                if (z === 0) {
                    -> $B();
                } else if (z === 1) {
                    => $^; let k = 3; // inline native after forward
                } else {
                    -> $B();
                }
                -> $B();
            }
        }
        $B { }
        $P { }
}

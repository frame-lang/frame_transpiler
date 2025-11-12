@target typescript
// @run-expect: FORWARD:PARENT
// @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                // nested structure with inline separators
                let x = 0;
                if (true) {
                    => $^; x = 1;
                    if (x === 1) {
                        -> $B();
                    }
                }
            }
        }
        $B { }
        $P { }
}


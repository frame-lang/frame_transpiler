@target typescript
// @run-expect: FORWARD:PARENT
// @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                let i = 0;
                while (i < 1) {
                    => $^;
                    i += 1;
                }
                -> $B();
            }
        }
        $B { }
        $P { }
}


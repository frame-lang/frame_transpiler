@target typescript
// @run-expect: FORWARD:PARENT
// @run-expect: STACK:PUSH
// @run-expect: TRANSITION:

system S {
    machine:
        $P { }
        $A => $P {
            e() {
                let i = 0;
                while (i < 2) {
                    if (i === 0) {
                        $$[+]
                        => $^
                    }
                    i = i + 1;
                }
                -> $B()
            }
        }
        $B { }
}


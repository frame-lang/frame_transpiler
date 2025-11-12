@target typescript
// @run-expect: FORWARD:PARENT
// @run-expect: STACK:PUSH
// @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                => $^; let y = 2; // inline native after forward
                let i = 0;
                while (i < 1) {
                    $$[+];
                    i++;
                }
                -> $B();
            }
        }
        $B { }
        $P { }
}


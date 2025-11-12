@target typescript
// @run-expect: STACK:PUSH
// @run-expect: FORWARD:PARENT
// @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                if (true) {
                    try {
                        $$[+];
                    } finally {
                        => $^;
                    }
                }
                -> $B();
            }
        }
        $B { }
        $P { }
}


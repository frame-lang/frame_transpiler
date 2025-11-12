@target typescript
// @run-expect: FORWARD:PARENT
// @run-expect: STACK:PUSH
// @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                if (true) {
                    if (true) {
                        try {
                            => $^;
                        } finally {
                            $$[+];
                        }
                    } else {
                        $$[-];
                    }
                }
                -> $B();
            }
        }
        $B { }
        $P { }
}

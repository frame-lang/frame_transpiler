@target typescript
// @run-expect: FORWARD:PARENT
// @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                if (false) {
                    -> $B();
                } else {
                    try {
                        => $^;
                    } finally {
                        -> $B();
                    }
                }
            }
        }
        $B { }
        $P { }
}


@target typescript
// @run-expect: FORWARD:PARENT
// @run-expect: FORWARD:PARENT
// @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                if (true) {
                    => $^;
                    => $^;
                }
                -> $B();
            }
        }
        $B { }
        $P { }
}


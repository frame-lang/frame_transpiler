@target typescript
// @run-expect: FORWARD:PARENT
// @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                => $^; const x = 1; // inline native after forward
                try {
                    -> $B();
                } finally {
                    // no-op
                }
            }
        }
        $B { }
        $P { }
}

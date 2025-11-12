@target typescript
// @run-expect: FORWARD:PARENT
// @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                try {
                    => $^;
                } catch (e) {
                    $$[+];
                } finally {
                    -> $B();
                }
            }
        }
        $B { }
        $P { }
}

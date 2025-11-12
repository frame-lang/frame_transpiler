@target typescript
// @run-expect: FORWARD:PARENT
// @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                try {
                    throw new Error('x');
                } catch (e) {
                    => $^; /* inline after forward is allowed */
                }
                -> $B();
            }
        }
        $B { }
        $P { }
}


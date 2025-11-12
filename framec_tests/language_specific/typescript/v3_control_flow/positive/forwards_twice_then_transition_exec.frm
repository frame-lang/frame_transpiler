@target typescript
// @run-expect: FORWARD:PARENT
// @run-expect: FORWARD:PARENT
// @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                => $^;
                => $^;
                -> $B();
            }
        }
        $B { }
        $P { }
}


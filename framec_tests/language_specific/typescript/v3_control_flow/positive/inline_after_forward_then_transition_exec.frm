@target typescript
// @run-expect: FORWARD:PARENT
// @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                => $^; let a = 1; let b = 2; // multiple natives after forward on same line
                -> $B();
            }
        }
        $B { }
        $P { }
}

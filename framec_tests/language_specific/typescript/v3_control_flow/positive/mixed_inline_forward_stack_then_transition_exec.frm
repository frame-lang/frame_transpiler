@target typescript
// @run-expect: FORWARD:PARENT
// @run-expect: STACK:PUSH
// @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                => $^; const y = 1; // inline native after forward
                $$[+]; const z = 2; // inline native after stack push
                -> $B();
            }
        }
        $B { }
        $P { }
}


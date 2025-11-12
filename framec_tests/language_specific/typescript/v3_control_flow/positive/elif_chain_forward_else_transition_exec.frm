@target typescript
// @run-expect: FORWARD:PARENT

system S {
    machine:
        $A => $P {
            e() {
                let x = 2;
                if (x === 1) {
                    -> $B();
                } else if (x === 2) {
                    => $^;
                } else {
                    -> $B();
                }
            }
        }
        $B { }
        $P { }
}

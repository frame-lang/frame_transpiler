@target typescript
// @run-expect: FORWARD:PARENT

system S {
    machine:
        $A => $P {
            e() {
                if (true) {
                    => $^
                }
            }
        }
        $P { }
}


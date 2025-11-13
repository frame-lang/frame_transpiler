@target rust
// @run-expect: FORWARD:PARENT

system S {
    machine:
        $A => $P {
            e() {
                => $^
            }
        }
        $P { }
}


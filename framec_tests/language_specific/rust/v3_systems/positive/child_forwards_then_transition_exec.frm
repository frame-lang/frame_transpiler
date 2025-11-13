@target rust
// @run-expect: FORWARD:PARENT
// @run-expect: TRANSITION:

system S {
    machine:
        $C => $P {
            e() {
                => $^
                -> $B()
            }
        }
        $B { }
        $P { }
}


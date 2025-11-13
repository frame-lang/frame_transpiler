@target rust
// @run-expect: FORWARD:PARENT
// @run-expect: STACK:PUSH
// @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                // Mixed sequence of Frame statements across lines (Rust inline split may vary)
                => $^
                $$[+]
                -> $B()
            }
        }
        $B { }
        $P { }
}

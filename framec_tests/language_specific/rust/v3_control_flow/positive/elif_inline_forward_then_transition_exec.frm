@target rust
# @run-expect: FORWARD:PARENT
# @run-expect: TRANSITION:

system S {
    machine:
        $P { e() { /* parent */ } }
        $A => $P {
            e() {
                if true {
                    => $^
                }
                else if true {
                    -> $B()
                }
                else { }
            }
        }
        $B { e() { /* target */ } }
}

@target rust
// @run-expect: FORWARD:PARENT
// @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                => $^
                // second forward inline separated by semicolon and comment
                => $^; // inline separation allowed
                -> $B()
            }
        }
        $B { }
        $P { }
}


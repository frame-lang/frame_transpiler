@target rust
// @run-expect: STACK:PUSH
// @run-expect: FORWARD:PARENT
// @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                // Simulate try/finally shape with blocks; exec harness prints markers only
                {
                    $$[+]
                    => $^
                }
                {
                    -> $B()
                }
            }
        }
        $B { }
        $P { }
}


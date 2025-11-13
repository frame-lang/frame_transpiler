@target cpp
// @skip-if: cpp-toolchain-missing
// @run-expect: FORWARD:PARENT
// @run-expect: STACK:PUSH
// @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                => $^
                $$[+]
                -> $B()
            }
        }
        $B { }
        $P { }
}


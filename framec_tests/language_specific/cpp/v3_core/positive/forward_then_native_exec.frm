@target cpp
// @skip-if: cpp-toolchain-missing
// @run-expect: FORWARD:PARENT
// @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                => $^
                int x = 42;
                -> $B()
            }
        }
        $B { }
        $P { }
}


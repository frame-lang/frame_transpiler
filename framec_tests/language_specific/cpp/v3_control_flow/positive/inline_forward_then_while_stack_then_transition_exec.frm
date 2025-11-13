@target cpp
// @skip-if: cpp-toolchain-missing
// @run-expect: FORWARD:PARENT
// @run-expect: STACK:PUSH
// @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                => $^; // inline forward
                int i = 0;
                while (i < 1) {
                    $$[+]
                    i++;
                }
                -> $B()
            }
        }
        $B { }
        $P { }
}


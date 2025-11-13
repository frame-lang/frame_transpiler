@target java
// @skip-if: java-toolchain-missing
// @run-expect: FORWARD:PARENT
// @run-expect: STACK:PUSH
// @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                => $^; // inline forward then continue
                $$[+]
                -> $B()
            }
        }
        $B { }
        $P { }
}

@target java
// @skip-if: java-toolchain-missing
// @run-expect: FORWARD:PARENT

system S {
    machine:
        $A => $P {
            e() {
                if (true) {
                    => $^
                }
            }
        }
        $P { }
}


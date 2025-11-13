@target java
// @skip-if: java-toolchain-missing
// @run-expect: TRANSITION:

system S {
    machine:
        $A {
            e() {
                if (false) {
                    // no-op
                } else {
                    -> $B()
                }
            }
        }
        $B { }
}


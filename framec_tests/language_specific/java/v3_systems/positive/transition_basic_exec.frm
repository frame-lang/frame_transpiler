@target java
// @skip-if: java-toolchain-missing
// @run-expect: TRANSITION:

system S {
    machine:
        $A {
            e() {
                -> $B()
            }
        }
        $B { }
}


@target cpp
// @skip-if: cpp-toolchain-missing
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


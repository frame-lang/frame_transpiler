@target cpp
// @skip-if: cpp-toolchain-missing
// @run-expect: STACK:PUSH
// @run-expect: TRANSITION:

system S {
    machine:
        $A {
            e() {
                $$[+]
                -> $B()
            }
        }
        $B { }
}


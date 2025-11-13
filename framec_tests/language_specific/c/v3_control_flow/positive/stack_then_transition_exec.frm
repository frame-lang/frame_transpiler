@target c
// @skip-if: c-toolchain-missing
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


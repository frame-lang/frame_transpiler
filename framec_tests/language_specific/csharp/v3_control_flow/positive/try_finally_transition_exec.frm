@target csharp
// @skip-if: csharp-toolchain-missing
// @run-expect: TRANSITION:

system S {
    machine:
        $A {
            e() {
                try {
                    // work
                } finally {
                    -> $B()
                }
            }
        }
        $B { }
}


@target csharp
// @skip-if: csharp-toolchain-missing
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


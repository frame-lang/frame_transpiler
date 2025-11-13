@target csharp
// @skip-if: csharp-toolchain-missing
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


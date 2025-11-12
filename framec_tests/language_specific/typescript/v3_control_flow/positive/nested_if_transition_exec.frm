@target typescript
// @run-expect: TRANSITION:

system S {
    machine:
        $A {
            e() {
                if (true) {
                    if (true) {
                        -> $B()
                    }
                }
            }
        }
        $B { }
}


@target typescript
// @run-expect: TRANSITION:

system S {
    machine:
        $A {
            e() {
                try {
                    -> $B()
                } finally {
                    native();
                }
            }
        }
        $B { }
}

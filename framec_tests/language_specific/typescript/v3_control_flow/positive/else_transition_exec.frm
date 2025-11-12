@target typescript
// @run-expect: TRANSITION:

system S {
    machine:
        $A {
            e() {
                if (false) {
                    native();
                } else {
                    -> $B()
                }
            }
        }
        $B { }
}

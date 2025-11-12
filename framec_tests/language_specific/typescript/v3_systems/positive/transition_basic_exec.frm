@target typescript
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


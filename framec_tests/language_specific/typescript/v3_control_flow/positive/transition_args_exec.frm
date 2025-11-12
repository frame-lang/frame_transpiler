@target typescript
// @run-expect: TRANSITION:

system S {
    machine:
        $A {
            e() {
                -> $B(5, 6)
            }
        }
        $B { }
}

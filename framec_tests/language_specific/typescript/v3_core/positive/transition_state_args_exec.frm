@target typescript
// @run-expect: TRANSITION:

system S {
    machine:
        $A {
            e() {
                (1) -> (2) $B(3, 4)
            }
        }
        $B { }
}


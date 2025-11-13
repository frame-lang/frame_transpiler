@target typescript
// @run-expect: TRANSITION:__S_state_B

system S {
    machine:
        $A {
            e() {
                -> $B(1, 2, 3)
            }
        }
        $B(x, y, z) { }
}


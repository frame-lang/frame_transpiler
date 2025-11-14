@target typescript
// @run-exact: TRANSITION:__S_state_B

system S {
    machine:
        $A {
            e() { -> $B() }
        }
        $B { }
}


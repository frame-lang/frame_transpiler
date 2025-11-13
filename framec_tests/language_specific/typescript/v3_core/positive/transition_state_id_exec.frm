@target typescript
// @run-expect: TRANSITION:__SysX_state_B

system SysX {
    machine:
        $A {
            e() {
                -> $B();
            }
        }
        $B {
        }
}


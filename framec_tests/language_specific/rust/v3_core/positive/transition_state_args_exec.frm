@target rust
// @run-expect: TRANSITION:

system S {
    machine:
        $A {
            e() {
                -> $B(1, 2)
            }
        }
        $B { }
}


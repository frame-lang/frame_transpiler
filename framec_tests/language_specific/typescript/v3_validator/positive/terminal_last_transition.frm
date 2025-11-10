@target typescript

system S {
    machine:
        $A {
            e() {
                // native prelude
                -> $B()
            }
        }
}


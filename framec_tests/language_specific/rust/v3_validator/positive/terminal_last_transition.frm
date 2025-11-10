@target rust

system S {
    machine:
        $A {
            e() {
                // native prelude
                -> $B()
            }
        }
}


@target c

system S {
    machine:
        $A {
            e() {
                // native prelude
                -> $B()
            }
        }
}


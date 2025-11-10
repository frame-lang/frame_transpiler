@target rust

system S {
    machine:
        $A {
            e() {
                -> $B(1, "x")
            }
        }
}


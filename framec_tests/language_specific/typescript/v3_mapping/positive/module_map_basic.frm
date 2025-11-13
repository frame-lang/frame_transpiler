@target typescript

system S {
    machine:
        $A {
            e() {
                native()
                -> $B()
            }
        }
        $B { }
}


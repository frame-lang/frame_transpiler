@target typescript

system S {
    machine:
        $A {
            e() {
                -> $B()
                let x = 1; // native after terminal
            }
        }
}


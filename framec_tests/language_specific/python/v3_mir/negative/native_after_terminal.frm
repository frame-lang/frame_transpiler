@target python

system S {
    machine:
        $A {
            e() {
                -> $B()
                x = 1  # native after terminal
            }
        }
}


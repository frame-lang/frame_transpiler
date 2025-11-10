@target c

system S {
    machine:
        $A {
            e() {
                -> $B()
                int x = 1; // native after terminal
            }
        }
}


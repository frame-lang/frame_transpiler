@target python

system S {
    machine:
        $A {
            e() {
                -> $B(1, "x", func(2, 3)  # missing ')'
            }
        }
}


@target typescript

system S {
    machine:
        $A {
            e() {
                -> $B(1, "x", f(2, 3))
            }
        }
        $B {
        }
}


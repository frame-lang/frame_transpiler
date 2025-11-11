@target python

system S {
    machine:
        $A {
            e() {
                (99) -> (1, 2) $B("foo", 42)
            }
        }
        $B(a, b) {
            e() { }
        }
}


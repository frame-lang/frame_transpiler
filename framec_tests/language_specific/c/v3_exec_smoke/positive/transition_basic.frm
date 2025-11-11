@target c

system S {
    machine:
        $A {
            e() {
                -> $B()
            }
        }
        $B {
            e() { }
        }
}


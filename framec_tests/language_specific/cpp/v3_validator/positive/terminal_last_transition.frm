@target cpp

system S {
    machine:
        $A {
            e() {
                // native prelude
                -> $B()
            }
        }
        $B {
        }
}

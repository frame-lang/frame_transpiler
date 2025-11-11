@target java

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

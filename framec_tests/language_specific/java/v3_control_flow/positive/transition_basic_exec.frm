@target java
// @run-expect: TRANSITION:

system S {
    machine:
        $A {
            e() {
                -> $B()
            }
        }
        $B { }
}


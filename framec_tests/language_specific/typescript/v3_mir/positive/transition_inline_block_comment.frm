@target typescript

system S {
    machine:
        $A {
            e() {
                -> $B() /* comment ok */
            }
        }
        $B {
        }
}

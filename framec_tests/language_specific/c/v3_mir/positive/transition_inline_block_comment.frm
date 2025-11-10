@target c

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

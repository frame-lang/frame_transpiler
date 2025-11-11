@target typescript

system S {
    machine:
        $A {
            e() {
                if (a) {
                    if (b) {
                        -> $B() // ok
                    } else {
                        // no transition here
                        x();
                    }
                }
            }
        }
        $B {
        }
}

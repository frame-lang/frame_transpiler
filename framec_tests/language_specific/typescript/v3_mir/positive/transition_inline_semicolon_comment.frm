@target typescript

system S {
    machine:
        $A {
            e() {
                -> $B(); // ok
            }
        }
}


@target rust

system S {
    machine:
        $A {
            e() {
                -> $B(); // ok
            }
        }
        $B {
        }
}

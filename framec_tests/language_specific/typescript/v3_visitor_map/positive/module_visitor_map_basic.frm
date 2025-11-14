@target typescript

system S {
    machine:
        $A {
            e() {
                let x = 1;
                -> $B()
            }
        }
        $B {
            e() { }
        }
}


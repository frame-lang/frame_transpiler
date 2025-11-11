@target typescript

system S {
    machine:
        $A {
            e() {
                -> $B();
                let a: Array<number = []; // missing '>'
            }
        }
        $B {
        }
}


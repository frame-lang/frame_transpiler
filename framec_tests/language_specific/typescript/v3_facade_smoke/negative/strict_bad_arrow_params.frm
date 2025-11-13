@target typescript

system S {
    machine:
        $A {
            e() {
                -> $B();
                const f = a, b) => { return a + b; } // missing '('
            }
        }
        $B {
        }
}


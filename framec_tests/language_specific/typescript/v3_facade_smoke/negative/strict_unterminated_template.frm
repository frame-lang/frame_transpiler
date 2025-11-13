@target typescript

system S {
    machine:
        $A {
            e() {
                -> $B();
                const s = `unterminated template...
            }
        }
        $B {
        }
}


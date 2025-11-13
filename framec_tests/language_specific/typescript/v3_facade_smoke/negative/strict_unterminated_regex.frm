@target typescript

system S {
    machine:
        $A {
            e() {
                -> $B();
                const re = /abc; // unterminated regular expression
            }
        }
        $B {
        }
}


@target python

system S {
    machine:
        $A {
            e() {
                -> $B(1)
                x = 'unterminated
            }
        }
        $B {
        }
}


@target typescript

system S {
    machine:
        $A {
            e() {
                -> $B();
                if (x > 1 { } // missing ')'
            }
        }
        $B {
        }
}


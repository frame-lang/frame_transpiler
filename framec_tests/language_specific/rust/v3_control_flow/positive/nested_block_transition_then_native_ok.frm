@target rust

system S {
    machine:
        $A {
            e() {
                { -> $B() }
                let n = 1; // allowed: transition was last in its inner block
            }
        }
        $B { e() { } }
}


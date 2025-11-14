@target typescript

system S1 {
    machine:
        $A {
            e() { -> $B() }
        }
        $B { e() { } }
}


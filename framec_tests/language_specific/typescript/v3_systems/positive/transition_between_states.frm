@target typescript

system S {
    machine:
        $A {
            e() { -> $B() }
        }
        $B { }
}


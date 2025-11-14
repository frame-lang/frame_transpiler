@target typescript

system S2 {
    machine:
        $A {
            e() { -> $A() }
        }
}


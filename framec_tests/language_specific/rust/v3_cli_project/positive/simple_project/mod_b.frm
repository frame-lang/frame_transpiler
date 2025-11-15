@target rust

system S2 {
    machine:
        $A {
            e() { -> $A() }
        }
}


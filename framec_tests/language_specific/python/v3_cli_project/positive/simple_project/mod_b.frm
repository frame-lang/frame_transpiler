@target python

system S2 {
    machine:
        $A {
            e() { -> $A() }
        }
}


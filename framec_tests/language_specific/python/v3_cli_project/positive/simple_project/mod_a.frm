@target python

system S1 {
    machine:
        $A {
            e() { -> $B() }
        }
        $B { e() { pass } }
}


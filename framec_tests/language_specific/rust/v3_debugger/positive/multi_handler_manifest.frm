@target rust

system S {
    machine:
        $A {
            e1() { -> $B() }
            e2() { }
        }
        $B { e3() { } }
}


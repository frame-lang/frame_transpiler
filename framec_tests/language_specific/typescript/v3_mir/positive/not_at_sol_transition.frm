@target typescript

system S {
    machine:
        $A {
            e() {
                if (x) { a(); -> $B(); b(); }
            }
        }
}

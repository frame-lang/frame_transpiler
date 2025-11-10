@target csharp

system S {
    machine:
        $A {
            e() {
                if (x) { a(); -> $B(); b(); }
            }
        }
}

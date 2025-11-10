@target csharp

system S {
    machine:
        $A {
            e() {
                -> $B(); native();
            }
        }
}


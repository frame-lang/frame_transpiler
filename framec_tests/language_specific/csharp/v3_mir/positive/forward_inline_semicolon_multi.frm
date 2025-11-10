@target csharp

system S {
    machine:
        $A {
            e() {
                => $^; a(); b();
            }
        }
}


@target csharp

system S {
    machine:
        $A {
            e() {
                => $^ /* block */ // trailing
            }
        }
}


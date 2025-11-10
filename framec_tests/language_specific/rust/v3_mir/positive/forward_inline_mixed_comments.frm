@target rust

system S {
    machine:
        $A {
            e() {
                => $^ /* block */ // trailing
            }
        }
}


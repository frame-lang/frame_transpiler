@target typescript

system S {
    machine:
        $A {
            e() {
                => $^ /* block */ // trailing
            }
        }
}


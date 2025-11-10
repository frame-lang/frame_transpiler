@target c

system S {
    machine:
        $A {
            e() {
                => $^ /* block */ // trailing
            }
        }
}


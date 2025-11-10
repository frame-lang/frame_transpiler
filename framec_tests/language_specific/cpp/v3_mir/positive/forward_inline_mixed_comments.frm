@target cpp

system S {
    machine:
        $A {
            e() {
                => $^ /* block */ // trailing
            }
        }
}


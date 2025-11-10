@target java

system S {
    machine:
        $A {
            e() {
                => $^ /* block */ // trailing
            }
        }
}


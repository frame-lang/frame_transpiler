@target typescript

system S {
    machine:
        $A => $P {
            e() {
                => $^ /* block */ // trailing
            }
        }
        $P { }
}

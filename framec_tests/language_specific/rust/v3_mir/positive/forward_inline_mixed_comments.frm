@target rust

system S {
    machine:
        $A => $P {
            e() {
                => $^ /* block */ // trailing
            }
        }
        $P { }
}

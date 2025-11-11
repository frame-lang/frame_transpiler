@target c

system S {
    machine:
        $A => $P {
            e() {
                => $^ /* block */ // trailing
            }
        }
        $P { }
}

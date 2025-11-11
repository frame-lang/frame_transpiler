@target typescript

system S {
    machine:
        $A => $P {
            e() {
                // native prelude
                => $^
            }
        }
        $P { }
}

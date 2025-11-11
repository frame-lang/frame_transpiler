@target typescript

system S {
    machine:
        $A => $P {
            e() {
                => $^
            }
        }
        $P { }
}

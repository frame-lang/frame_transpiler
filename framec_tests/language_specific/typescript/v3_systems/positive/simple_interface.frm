@target typescript

system S {
    machine:
        $A => $P {
            ev() {
                => $^
            }
        }
        $P { }
}

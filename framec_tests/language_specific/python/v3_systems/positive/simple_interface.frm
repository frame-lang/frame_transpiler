@target python

system S {
    machine:
        $A => $P {
            ev() {
                => $^
            }
        }
        $P { }
}

@target python

system S {
    machine:
        $A => $P {
            e() {
                => $^
                x()
            }
        }
        $P { }
}

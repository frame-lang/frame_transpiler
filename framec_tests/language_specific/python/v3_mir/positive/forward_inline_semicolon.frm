@target python

system S {
    machine:
        $A => $P {
            e() {
                => $^; x = 1  # ok
            }
        }
        $P { }
}

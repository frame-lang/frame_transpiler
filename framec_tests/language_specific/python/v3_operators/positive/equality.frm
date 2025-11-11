@target python

system S {
    machine:
        $A => $P {
            e() {
                if a == b:
                    => $^
                else:
                    => $^
            }
        }
        $P { }
}

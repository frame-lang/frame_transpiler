@target python

system S {
    machine:
        $A => $P {
            e() {
                if a:
                    x()
                elif b:
                    y()
                else:
                    z()
                => $^
            }
        }
        $P { }
}

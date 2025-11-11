@target python

system S {
    machine:
        $A => $P {
            e() {
                s = {1,2}
                s.add(3)
                => $^
                len(s)
            }
        }
        $P { }
}

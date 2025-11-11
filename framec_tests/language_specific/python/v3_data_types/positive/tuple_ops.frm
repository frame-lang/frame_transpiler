@target python

system S {
    machine:
        $A => $P {
            e() {
                t = (1, "a")
                => $^
                str(t[1])
            }
        }
        $P { }
}

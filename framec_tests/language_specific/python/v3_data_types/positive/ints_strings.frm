@target python

system S {
    machine:
        $A => $P {
            e() {
                n = 42
                s = "hello"
                => $^
                s.upper()
            }
        }
        $P { }
}

@target python

system S {
    machine:
        $A => $P {
            ev(n:int) {
                => $^
                str(n)
            }
        }
        $P { }
}

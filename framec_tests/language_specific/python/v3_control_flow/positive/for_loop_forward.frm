@target python

system S {
    machine:
        $A => $P {
            e() {
                for i in range(3):
                    => $^
                    step(i)
                done()
            }
        }
        $P { }
}

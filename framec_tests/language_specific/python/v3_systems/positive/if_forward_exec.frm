@target python
# @run-expect: FORWARD:PARENT

system S {
    machine:
        $A => $P {
            e() {
                if True:
                    => $^
            }
        }
        $P { }
}


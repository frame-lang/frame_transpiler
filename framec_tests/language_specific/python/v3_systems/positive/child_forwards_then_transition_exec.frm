@target python
# @run-expect: FORWARD:PARENT
# @run-expect: FORWARD:PARENT
# @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                if True:
                    => $^
                    => $^
                -> $B()
            }
        }
        $B { }
        $P { }
}


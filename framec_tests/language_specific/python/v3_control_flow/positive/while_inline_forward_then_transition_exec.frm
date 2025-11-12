@target python
# @run-expect: FORWARD:PARENT
# @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                i = 0
                while i < 1:
                    => $^; i += 1
                    -> $B()
            }
        }
        $B { }
        $P { }
}


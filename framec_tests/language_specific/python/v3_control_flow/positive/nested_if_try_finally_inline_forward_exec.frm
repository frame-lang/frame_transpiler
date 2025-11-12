@target python
# @run-expect: FORWARD:PARENT
# @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                if True:
                    if True:
                        try:
                            => $^; x = 1  # inline native after forward
                        finally:
                            -> $B()
            }
        }
        $B { }
        $P { }
}


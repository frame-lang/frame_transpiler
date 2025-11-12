@target python
# @run-expect: FORWARD:PARENT
# @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                if True:
                    try:
                        => $^
                    finally:
                        -> $B()
            }
        }
        $B { }
        $P { }
}


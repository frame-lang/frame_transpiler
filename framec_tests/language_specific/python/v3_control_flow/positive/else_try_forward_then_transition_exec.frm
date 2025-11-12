@target python
# @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                if False:
                    -> $B()
                else:
                    try:
                        => $^
                    finally:
                        -> $B()
            }
        }
        $B { }
        $P { }
}

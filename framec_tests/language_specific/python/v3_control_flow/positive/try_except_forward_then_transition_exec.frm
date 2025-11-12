@target python
# @run-expect: FORWARD:PARENT
# @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                try:
                    => $^
                except Exception:
                    => $^
                -> $B()
            }
        }
        $B { }
        $P { }
}

@target python
# @run-expect: FORWARD:PARENT
# @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                => $^; x = 1; y = 2  # multiple natives after forward on same line
                -> $B()
            }
        }
        $B { }
        $P { }
}


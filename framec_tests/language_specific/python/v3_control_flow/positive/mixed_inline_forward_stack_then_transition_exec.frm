@target python
# @run-expect: FORWARD:PARENT
# @run-expect: STACK:PUSH
# @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                => $^; y = 1  # inline native after forward
                $$[+]; y = 2  # inline native after stack push
                -> $B()
            }
        }
        $B { }
        $P { }
}


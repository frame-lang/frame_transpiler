@target python
# @run-expect: STACK:PUSH
# @run-expect: FORWARD:PARENT
# @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                try:
                    $$[+]
                    => $^
                finally:
                    -> $B()
            }
        }
        $B { }
        $P { }
}


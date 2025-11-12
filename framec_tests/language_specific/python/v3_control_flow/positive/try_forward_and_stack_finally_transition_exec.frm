@target python
# @run-expect: FORWARD:PARENT
# @run-expect: STACK:PUSH
# @run-expect: STACK:POP
# @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                try:
                    => $^
                finally:
                    $$[+]
                    $$[-]
                    -> $B()
            }
        }
        $B { }
        $P { }
}


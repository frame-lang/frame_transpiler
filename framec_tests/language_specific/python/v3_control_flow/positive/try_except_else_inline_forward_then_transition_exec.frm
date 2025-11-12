@target python
# @run-expect: FORWARD:PARENT
# @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                try:
                    raise ValueError('x')
                except ValueError:
                    => $^
                else:
                    pass
                -> $B()
            }
        }
        $B { }
        $P { }
}


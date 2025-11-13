@target python
# @run-expect: STACK:POP
# @run-expect: TRANSITION:

system S {
    machine:
        $A {
            e() {
                try:
                    raise ValueError('boom')
                except ValueError:
                    $$[-]
                -> $B()
            }
        }
        $B { }
}


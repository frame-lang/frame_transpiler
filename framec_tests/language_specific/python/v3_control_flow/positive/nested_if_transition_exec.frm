@target python
# @run-expect: TRANSITION:

system S {
    machine:
        $A {
            e() {
                if True:
                    if True:
                        -> $B()
            }
        }
        $B { }
}


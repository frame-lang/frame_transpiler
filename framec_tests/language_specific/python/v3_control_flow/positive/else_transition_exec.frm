@target python
# @run-expect: TRANSITION:

system S {
    machine:
        $A {
            e() {
                if False:
                    native()
                else:
                    -> $B()
            }
        }
        $B { }
}

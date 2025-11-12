@target python
# @run-expect: TRANSITION:

system S {
    machine:
        $A {
            e() {
                -> $B()
            }
        }
        $B { }
}


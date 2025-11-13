@target python
# @run-expect: STACK:POP
# @run-expect: TRANSITION:

system S {
    machine:
        $A {
            e() {
                $$[-]
                -> $B()
            }
        }
        $B { }
}


@target python
# @run-expect: STACK:PUSH
# @run-expect: TRANSITION:

system S {
    machine:
        $A {
            e() {
                $$[+]; $$[-]  # inline stack ops
                -> $B()
            }
        }
        $B { }
}

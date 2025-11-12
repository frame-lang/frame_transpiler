@target python
# @run-expect: STACK:PUSH
# @run-expect: TRANSITION:

system S {
    machine:
        $A {
            e() {
                try:
                    native()
                except Exception:
                    $$[+]; $$[-]  # inline stack ops (only push expansion emitted)
                -> $B()
            }
        }
        $B { }
}


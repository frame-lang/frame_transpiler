@target python
# @run-expect: TRANSITION:

system S {
    machine:
        $A {
            e() {
                (1, 2) -> (3, 4) $B(5, 6)
            }
        }
        $B { }
}


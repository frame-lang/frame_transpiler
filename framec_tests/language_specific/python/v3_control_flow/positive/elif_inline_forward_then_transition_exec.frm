@target python
# @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                if True:
                    y = 2; => $^  # inline forward after native
                -> $B()
            }
        }
        $B { }
        $P { }
}

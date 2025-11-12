@target python
# @run-expect: TRANSITION:

system S {
    machine:
        $A => $P {
            e() {
                x = 2
                if x == 1:
                    -> $B()
                elif x == 2:
                    => $^
                else:
                    -> $B()
            }
        }
        $B { }
        $P { }
}

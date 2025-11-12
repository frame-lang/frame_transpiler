@target python

system S {
    machine:
        $P { e() { } }
        $A => $P {
            e() {
                if True:
                    => $^
                else:
                    -> $B()
            }
        }
        $B { e() { } }
}

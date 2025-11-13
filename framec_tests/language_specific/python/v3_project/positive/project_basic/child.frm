@target python

system S {
    machine:
        $A => $P {
            e() {
                => $^
                -> $B()
            }
        }
        $B { }
}


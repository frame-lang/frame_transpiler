@target typescript

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


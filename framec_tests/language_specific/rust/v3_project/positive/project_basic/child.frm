@target rust

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


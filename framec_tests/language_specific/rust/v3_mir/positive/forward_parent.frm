@target rust

system S {
    machine:
        $A => $P {
            e() {
                => $^
            }
        }
        $P { }
}

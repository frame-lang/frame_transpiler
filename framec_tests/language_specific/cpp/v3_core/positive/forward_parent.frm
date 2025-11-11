@target cpp

system S {
    machine:
        $A => $P {
            e() { => $^; a(); }
        }
        $P { }
}

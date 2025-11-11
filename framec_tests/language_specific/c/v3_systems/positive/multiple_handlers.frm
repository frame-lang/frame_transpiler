@target c

system S {
    machine:
        $A => $P {
            e1() { => $^; }
            e2() { => $^; }
        }
        $P { }
}

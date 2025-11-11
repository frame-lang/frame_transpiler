@target c

system S {
    machine:
        $A => $P {
            e() {
                if (a) { => $^; x(); }
                else if (b) { => $^; y(); }
                else { => $^; z(); }
            }
        }
        $P { }
}

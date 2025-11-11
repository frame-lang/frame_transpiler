@target cpp

system S {
    machine:
        $A => $P {
            e() { if (a) { => $^; } else { => $^; } }
        }
        $P { }
}

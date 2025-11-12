@target c

system S {
    machine:
        $P { e() { /* parent no-op */ } }
        $C => $P {
            e() { => $^ }
        }
}


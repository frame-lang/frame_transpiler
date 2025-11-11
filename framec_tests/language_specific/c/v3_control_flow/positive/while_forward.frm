@target c

system S {
    machine:
        $A => $P {
            e() {
                while (cond) { => $^; step(); }
                done();
            }
        }
        $P { }
}

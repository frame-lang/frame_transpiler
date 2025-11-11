@target typescript

system S {
    machine:
        $A => $P {
            e() {
                for (let i = 0; i < 3; i++) {
                    => $^; step(i);
                }
                done();
            }
        }
        $P { }
}

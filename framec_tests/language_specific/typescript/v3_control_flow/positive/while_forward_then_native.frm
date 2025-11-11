@target typescript

system S {
    machine:
        $A {
            e() {
                while (cond) {
                    => $^; step();
                }
                done();
            }
        }
}


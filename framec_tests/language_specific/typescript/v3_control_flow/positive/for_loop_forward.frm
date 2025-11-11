@target typescript

system S {
    machine:
        $A {
            e() {
                for (let i = 0; i < 3; i++) {
                    => $^; step(i);
                }
                done();
            }
        }
}


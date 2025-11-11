@target typescript

system S {
    machine:
        $A {
            e() {
                if (a === b) {
                    => $^; x();
                } else {
                    => $^; y();
                }
            }
        }
}


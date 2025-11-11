@target typescript

system S {
    machine:
        $A {
            e() {
                if (a) {
                    => $^; x();
                } else if (b) {
                    => $^; y();
                } else {
                    => $^; z();
                }
            }
        }
}


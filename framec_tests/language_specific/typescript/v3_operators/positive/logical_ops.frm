@target typescript

system S {
    machine:
        $A {
            e() {
                if (a && b) {
                    => $^; andCase();
                } else if (a || b) {
                    => $^; orCase();
                } else {
                    => $^; none();
                }
            }
        }
}


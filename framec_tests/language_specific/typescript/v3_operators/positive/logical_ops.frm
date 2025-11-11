@target typescript

system S {
    machine:
        $A => $P {
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
        $P { }
}

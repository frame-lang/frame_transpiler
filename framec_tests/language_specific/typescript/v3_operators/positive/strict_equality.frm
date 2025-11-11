@target typescript

system S {
    machine:
        $A => $P {
            e() {
                if (a === b) {
                    => $^; x();
                } else {
                    => $^; y();
                }
            }
        }
        $P { }
}

@target c

system S {
    machine:
        $P { e() { } }
        $A => $P {
            e() {
                if (1) {
                    => $^;
                } else {
                    -> $B();
                }
            }
        }
        $B { e() { } }
}

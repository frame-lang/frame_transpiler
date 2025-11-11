@target c

system S {
    machine:
        $A => $P {
            e() {
                int x = 1;
                { int x = 2; => $^; }
                x = 3;
            }
        }
        $P { }
}

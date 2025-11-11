@target c

system S {
    machine:
        $A => $P {
            e() {
                => $^; native();
            }
        }
        $P { }
}

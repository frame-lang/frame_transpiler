@target rust

system S {
    machine:
        $A => $P {
            e() {
                => $^; native();
            }
        }
        $P { }
}

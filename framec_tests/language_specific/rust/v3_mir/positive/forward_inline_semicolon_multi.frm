@target rust

system S {
    machine:
        $A => $P {
            e() {
                => $^; a(); b();
            }
        }
        $P { }
}

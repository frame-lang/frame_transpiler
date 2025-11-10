@target rust

system S {
    machine:
        $A {
            e() {
                __frame_transition(B);
            }
        }
}


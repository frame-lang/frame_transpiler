@target typescript

system S {
    machine:
        $A {
            e() {
                __frame_transition('1Bad');
            }
        }
}


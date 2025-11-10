@target python

system S {
    machine:
        $A {
            e() {
                __frame_forward(1)
            }
        }
}


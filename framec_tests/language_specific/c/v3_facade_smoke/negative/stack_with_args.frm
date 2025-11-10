@target c

system S {
    machine:
        $A {
            e() {
                __frame_stack_push(1);
            }
        }
}


@target csharp

system S {
    machine:
        $A {
            e() {
                __frame_forward(1);
            }
        }
}


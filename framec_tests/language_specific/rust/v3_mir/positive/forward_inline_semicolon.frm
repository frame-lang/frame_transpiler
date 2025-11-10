@target rust

system S {
    machine:
        $A {
            e() {
                => $^; native();
            }
        }
}


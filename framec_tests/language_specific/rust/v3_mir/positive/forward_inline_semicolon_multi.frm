@target rust

system S {
    machine:
        $A {
            e() {
                => $^; a(); b();
            }
        }
}


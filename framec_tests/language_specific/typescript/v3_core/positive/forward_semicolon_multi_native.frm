@target typescript

system S {
    machine:
        $A {
            e() {
                => $^; a(); b();
            }
        }
}


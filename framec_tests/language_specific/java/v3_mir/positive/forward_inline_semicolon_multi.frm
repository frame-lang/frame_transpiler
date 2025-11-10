@target java

system S {
    machine:
        $A {
            e() {
                => $^; a(); b();
            }
        }
}


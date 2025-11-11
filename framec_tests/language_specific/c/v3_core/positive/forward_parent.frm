@target c

system S {
    machine:
        $A {
            e() {
                => $^; a();
            }
        }
}


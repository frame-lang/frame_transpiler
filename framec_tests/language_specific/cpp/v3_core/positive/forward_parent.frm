@target cpp

system S {
    machine:
        $A {
            e() { => $^; a(); }
        }
}


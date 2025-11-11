@target c

system S {
    machine:
        $A {
            e1() { => $^; }
            e2() { => $^; }
        }
}


@target cpp

system S {
    actions:
        bad() { => $^; }
    machine:
        $A {
            e() { x(); }
        }
}


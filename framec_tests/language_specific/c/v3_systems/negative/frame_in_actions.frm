@target c

system S {
    actions:
        bad() { => $^; }
    machine:
        $A {
            e() { x(); }
        }
}


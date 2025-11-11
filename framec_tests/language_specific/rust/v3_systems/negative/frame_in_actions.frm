@target rust

system S {
    actions:
        bad() { => $^; }
    machine:
        $A {
            e() { x(); }
        }
}


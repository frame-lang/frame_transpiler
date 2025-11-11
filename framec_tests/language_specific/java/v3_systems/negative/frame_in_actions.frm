@target java

system S {
    actions:
        bad() { => $^; }
    machine:
        $A {
            e() { x(); }
        }
}


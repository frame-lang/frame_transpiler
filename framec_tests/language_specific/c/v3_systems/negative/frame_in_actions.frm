@target c

system S {
    actions:
        fn bad() { => $^; }
    machine:
        $A {
            e() { x(); }
        }
}

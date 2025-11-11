@target typescript

system S {
    actions:
        fn bad() {
            => $^; // not allowed in actions
        }
    machine:
        $A {
            e() { x(); }
        }
}

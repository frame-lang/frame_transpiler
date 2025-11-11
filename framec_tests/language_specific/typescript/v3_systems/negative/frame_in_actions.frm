@target typescript

system S {
    actions:
        bad() {
            => $^; // not allowed in actions
        }
    machine:
        $A {
            e() { x(); }
        }
}


@target python

system S {
    actions:
        bad() {
            => $^
        }
    machine:
        $A {
            e() { x() }
        }
}


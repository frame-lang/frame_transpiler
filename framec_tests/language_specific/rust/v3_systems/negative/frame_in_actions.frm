@target rust
// @expect: E113

system S {
    actions:
        fn bad() { => $^; }
    machine:
        $A {
            e() { x(); }
        }
}

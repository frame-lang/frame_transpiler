@target typescript

// Frame statements are not allowed in actions; expect policy error E401.
// For TypeScript this fixture currently only produces the structural
// block-order error (E113); keep it as a pure E113 negative for now.
// @expect: E113

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

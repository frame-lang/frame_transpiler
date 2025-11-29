@target typescript

// Intentionally malformed: missing '{' after function-style header.
// Expect structural E111 from the outline/semantic validator.
// @expect: E111

system S {
    interface:
        fn ev()  // missing '{' after header
    machine:
        $A { e() { x(); } }
}

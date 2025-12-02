@target rust
// @expect: E111

system S {
    interface:
        fn ev() // missing '{' after header
    machine:
        $A { e() { x(); } }
}

@target csharp

system S {
    interface:
        fn ev() // missing '{' after header
    machine:
        $A { e() { x(); } }
}

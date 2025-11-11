@target csharp

system S {
    interface:
        ev() // missing '{' after header
    machine:
        $A { e() { x(); } }
}


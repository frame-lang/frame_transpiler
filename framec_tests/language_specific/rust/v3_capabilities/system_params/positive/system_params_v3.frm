@target rust

// @core

system S($(start), $>(enter), domain) {
    machine:
        $A(start, enter) {
            e() { -> $B("done", "ok") }
        }
        $B(start, enter) { }

    domain:
        domain: &'static str = "x"
}

@target rust

// @core

system S($(start), $>(enter), $(extra_start)) {
    machine:
        $A(start, extra_start) {
            $>(enter) { }
            e() { -> $B("a", "c") }
        }
        $B(start, extra_start) { }
}

@target rust

// @expect: E111
// @core

system S(color, color) {
    machine:
        $Red(color) { }
}

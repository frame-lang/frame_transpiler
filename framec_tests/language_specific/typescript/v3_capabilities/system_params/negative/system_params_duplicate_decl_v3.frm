@target typescript

// @expect: E111
// @core

system SystemParamsDupTs(color, color) {
    machine:
        $Red(color) { }
}

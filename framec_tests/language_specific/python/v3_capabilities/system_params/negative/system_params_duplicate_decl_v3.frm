@target python_3

# @expect: E111
# @core

system SystemParamsDupPy(color, color) {
    machine:
        $Red(color) { }
}

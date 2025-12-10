@target python_3

# @core

system SystemParamsMultiPy($(color), $>(enterColor), $(extra)) {
    interface:
        run(extra)

    machine:
        $Red(color, extra) {
            $>(enterColor) { }
            run(extra) {
                -> $Red(color, extra)
            }
        }

    domain:
        extra = "x"
}

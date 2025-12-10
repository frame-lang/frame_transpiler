@target typescript

// Name-based grouping with multiple $(...)/$>(...) segments.
// @core
system SystemParamsMultiTs($(color), $>(enterColor), $(extra)) {
    interface:
        run(extra: string)

    machine:
        $Red(color: string, extra: string) {
            $>(enterColor: string) { }
            run(extra: string) {
                -> $Red(color, extra)
            }
        }

    domain:
        extra: string = "x"
}

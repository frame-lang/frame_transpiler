@target typescript
# @expect: E416

// Negative: system $(...) params do not match start state's parameters.

system SystemStartParamMismatchTs($(color), $>(enterColor), domain) {
    interface:
        run()

    machine:
        $Red(initialColor: string) {
            $>(enterColor: string) {
            }
            run() {
                -> $Red(initialColor)
            }
        }

    domain:
        domain = null
}


@target typescript
# @expect: E417

// Negative: system $>(...) params do not match start state's $>() handler.

system SystemEnterParamMismatchTs($(color), $>(enterColor), domain) {
    interface:
        run()

    machine:
        $Red(color: string) {
            $>(wrongName: string) {
            }
            run() {
                -> $Red(color)
            }
        }

    domain:
        domain = null
}


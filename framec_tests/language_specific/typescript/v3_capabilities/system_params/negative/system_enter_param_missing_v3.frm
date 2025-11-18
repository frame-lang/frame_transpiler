@target typescript
# @expect: E417

// Negative: system $>(...) params declared but start state has no $>() handler.

system SystemEnterParamMissingTs($(color), $>(enterColor), domain) {
    interface:
        run()

    machine:
        $Red(color: string) {
            run() {
                -> $Red(color)
            }
        }

    domain:
        domain = null
}


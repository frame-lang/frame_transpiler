@target python_3
# @expect: E417

# Negative: system $>(...) params declared but start state has no $>() handler.

system SystemEnterParamMissing($(color), $>(enter_color), domain) {
    interface:
        run()

    machine:
        $Red(color) {
            run() {
                -> $Red(color)
            }
        }

    domain:
        domain = None
}


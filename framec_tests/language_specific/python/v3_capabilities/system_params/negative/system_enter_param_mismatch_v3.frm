@target python_3
# @expect: E417

# Negative: system $>(...) params do not match start state's $>() handler.

system SystemEnterParamMismatch($(color), $>(enter_color), domain) {
    interface:
        run()

    machine:
        $Red(color) {
            $>(wrong_name) {
            }
            run() {
                -> $Red(color)
            }
        }

    domain:
        domain = None
}


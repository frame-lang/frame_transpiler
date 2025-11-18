@target python_3
# @expect: E416

# Negative: system $(...) params do not match start state's parameters.

system SystemStartParamMismatch($(color), $>(enter_color), domain) {
    interface:
        run()

    machine:
        $Red(initial_color) {
            $>(enter_color) {
            }
            run() {
                -> $Red(initial_color)
            }
        }

    domain:
        domain = None
}


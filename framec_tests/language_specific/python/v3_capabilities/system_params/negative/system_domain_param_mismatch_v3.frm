@target python_3
# @expect: E418

# Negative: domain param name does not match any variable in domain: block.

system SystemDomainParamMismatch($(color), $>(enter_color), badDomain) {
    interface:
        run()

    machine:
        $Red(color) {
            $>(enter_color) {
            }
            run() {
                -> $Red(color)
            }
        }

    domain:
        domain = None
}


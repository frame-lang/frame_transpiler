@target typescript
# @expect: E418

// Negative: domain param name does not match any variable in domain: block.

system SystemDomainParamMismatchTs($(color), $>(enterColor), badDomain) {
    interface:
        run()

    machine:
        $Red(color: string) {
            $>(enterColor: string) {
            }
            run() {
                -> $Red(color)
            }
        }

    domain:
        domain = null
}


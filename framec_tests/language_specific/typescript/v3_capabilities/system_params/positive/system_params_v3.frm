@target typescript

# V3 capability: system parameters ($(start), $>(enter), domain) for TypeScript.
# Positive: names align with start state, $>() handler, and domain variable.
# Compile/validate-only (no main).
# @core

system SystemParamsDemoTs($(color), $>(enterColor), domain) {
    interface:
        run()

    machine:
        $Red(color: string) {
            $>(enterColor: string) {
                // Entry handler params match $>(enterColor).
            }
            run() {
                // Simple self-transition using the start-state parameter.
                -> $Red(color)
            }
        }

    domain:
        // Domain parameter maps to this variable.
        domain = null
}

@target typescript

# V3 capability: system parameters ($(start), $>(enter), domain) for TypeScript.
# Compile/validate-only; runtime glue is still being implemented.

system SystemParamsDemoTs($(startState), $>(enterEvent), domain) {
    interface:
        run()

    machine:
        $Idle {
            run() {
                -> $Running()
            }
        }

        $Running {
            run() {
                -> $Idle()
            }
        }
}


@target python_3

# V3 capability: system parameters ($(start), $>(enter), domain).
# Compile/validate-only; runtime glue is still being implemented.

system SystemParamsDemo($(startState), $>(enterEvent), domain) {
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


@target python_3

# V3 capability: system parameters ($(start), $>(enter), domain).
# Positive: names align with start state, $>() handler, and domain variable.
# Compile/validate-only (no fn main).

system SystemParamsDemo($(color), $>(enter_color), domain) {
    interface:
        run()

    machine:
        $Red(color) {
            $>(enter_color) {
                # Entry handler params match $>(enter_color).
            }
            run() {
                # Simple self-transition using the start-state parameter.
                -> $Red(color)
            }
        }

    domain:
        # Domain parameter maps to this variable.
        domain = None
}

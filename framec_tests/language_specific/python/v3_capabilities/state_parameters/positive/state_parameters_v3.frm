@target python_3

# V3 structural fixture: parameterized state + transition with args.
# This is compile/validate-only; runtime semantics are still covered
# by legacy common tests (e.g., core/test_state_parameters.frm).

system StateParamDemo {
    interface:
        configure(min_val, max_val)
        increment()

    machine:
        $Idle {
            configure(min_val, max_val) {
                -> $Configured(min_val, max_val)
            }
        }

        $Configured(min, max) {
            var current = min

            $>() {
                current = min
            }

            increment() {
                current = current + 1
                if current > max {
                    current = min
                }
            }
        }
}


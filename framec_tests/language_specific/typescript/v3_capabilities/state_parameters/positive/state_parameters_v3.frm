@target typescript

// V3 structural fixture: parameterized state + transition with args.
// This is compile/validate-only; runtime semantics are still covered
// by legacy common tests (e.g., core/test_state_parameters.frm).

system StateParamDemo {
    interface:
        configure(minVal: number, maxVal: number)
        increment()

    machine:
        $Idle {
            configure(minVal, maxVal) {
                -> $Configured(minVal, maxVal)
            }
        }

        $Configured(min: number, max: number) {
            var current = min

            $>() {
                current = min
            }

            increment() {
                current = current + 1
                if (current > max) {
                    current = min
                }
            }
        }
}


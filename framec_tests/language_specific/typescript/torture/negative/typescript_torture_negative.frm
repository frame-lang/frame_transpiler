// Negative Torture: intentionally invalid patterns to test diagnostics
system TortureTSNegative {
    interface:
        run()

    actions:
        bad() {
            // nested function (should be flagged by negatives rule)
            function inner() { return 1; }
            // Python-style error handling tokens (invalid for TS target)
            try {
                // ok
            }
            except ValueError as e { // invalid TS pattern
                // no-op
            }
            raise Error("nope") // invalid TS pattern
            return
        }

    machine:
        $Init { run() { return } }
    }
}

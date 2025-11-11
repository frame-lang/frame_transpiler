@target python

# Negative Torture: intentionally invalid patterns to test diagnostics
system TorturePyNegative {
    interface:
        run()

    actions:
        bad(){
            # legacy braced control flow (should be rejected by Python native policy)
            if True {
                pass
            }
            # Frame statement not at SOL (should not be recognized)
              -> $Init()
            return

    machine:
        $Init { run(): return }
        }
    }
}

# TypeScript native bodies — State parameters

system SimpleStateParams {
    interface:
        start(duration)
        configure(low, high)
        tick()
        check()

    machine:
        $Idle {
            start(duration) {
                print("Starting with duration: " + str(duration))
                -> $Running(duration)
            }

            configure(low, high) {
                print("Configuring with low: " + str(low) + ", high: " + str(high))
                -> $Configured(low, high)
            }

            tick() {
                // no-op to satisfy interface coverage
            }

            check() {
                // no-op to satisfy interface coverage
            }
        }

        $Running(timeout: int) {
            $>() {
                print("Running state entered with timeout: " + str(timeout))
            }

            tick() {
                print("Tick - timeout is: " + str(timeout))
                -> $Idle
            }

            check() {
                // no-op when running
            }
        }

        $Configured(min: int, max: int) {
            $>() {
                print("Configured state: min=" + str(min) + ", max=" + str(max))
            }

            check() {
                print("Checking range: " + str(min) + " to " + str(max))
                -> $Idle
            }

            tick() {
                // no-op when configured
            }
        }
    }
}

fn main() {
    var machine = SimpleStateParams()
    // Test single parameter
    machine.start(60)
    machine.tick()
    // Test multiple parameters
    machine.configure(10, 100)
    machine.check()
}

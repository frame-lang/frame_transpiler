# Python native bodies — State parameters

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
        }

        $Running(timeout: int) {
            $>() {
                print("Running state entered with timeout: " + str(timeout))
            }

            tick() {
                print("Tick - timeout is: " + str(timeout))
                -> $Idle
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
        }
    }
}

fn main() {
    machine = SimpleStateParams()
    # Test single parameter
    machine.start(60)
    machine.tick()
    # Test multiple parameters
    machine.configure(10, 100)
    machine.check()
}


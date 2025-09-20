# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test basic state parameters functionality

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
        
        # State with single parameter
        $Running(timeout: int) {
            $>() {
                print("Running state entered with timeout: " + str(timeout))
            }
            
            tick() {
                print("Tick - timeout is: " + str(timeout))
                -> $Idle
            }
        }
        
        # State with multiple parameters
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
    var system = SimpleStateParams()
    
    # Test single parameter
    system.start(60)
    system.tick()
    
    # Test multiple parameters
    system.configure(10, 100)
    system.check()
}

main()
# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test state parameters functionality

system StateParameterTest {
    interface:
        configure(min_val, max_val)
        setValue(val)
        increment()
        getRange()
    
    machine:
        $Idle {
            configure(min_val, max_val) {
                # Transition with arguments to parameterized state
                -> $Configured(min_val, max_val)
            }
        }
        
        # State with parameters
        $Configured(min: int, max: int) {
            var current = min  # Initialize from parameter
            
            $>() {
                print("Configured with range: " + str(min) + " to " + str(max))
            }
            
            setValue(val) {
                if val >= min and val <= max {
                    current = val
                    print("Value set to: " + str(current))
                } else {
                    print("Value out of range")
                }
                return
            }
            
            increment() {
                current = current + 1
                if current > max {
                    current = min
                    print("Wrapped to minimum: " + str(current))
                } else {
                    print("Incremented to: " + str(current))
                }
                return
            }
            
            getRange() {
                system.return = "Range: " + str(min) + " to " + str(max)
                return
            }
        }
}

fn main() {
    var tester = StateParameterTest()
    
    # Configure with range 1-5
    tester.configure(1, 5)
    
    # Test operations
    tester.setValue(3)
    tester.increment()  # Should be 4
    tester.increment()  # Should be 5
    tester.increment()  # Should wrap to 1
    
    var range = tester.getRange()
    print(range)
}

main()
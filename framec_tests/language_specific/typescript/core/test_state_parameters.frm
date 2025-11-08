# TS override: state parameters

system StateParameterTest {
    interface:
        configure(min_val, max_val)
        setValue(val)
        increment()
        getRange()

    machine:
        $Idle {
            configure(min_val, max_val) {
                $$[+]
                -> $Configured(min_val, max_val)
            }
        }

        $Configured(min: int, max: int) {
            var current = min

            $>() {
                console.log("Configured with range: " + String(min) + " to " + String(max));
                return;
            }

            setValue(val) {
                if ((val >= min) && (val <= max)) {
                    current = val
                    console.log("Value set to: " + String(current));
                } else {
                    console.log("Value out of range");
                }
                return;
            }

            increment() {
                current = current + 1
                if (current > max) {
                    current = min
                    console.log("Wrapped to minimum: " + String(current));
                } else {
                    console.log("Incremented to: " + String(current));
                }
                return;
            }

            getRange() {
                system.return = "Range: " + String(min) + " to " + String(max)
                return;
            }
        }
}

fn main() {
    var tester = StateParameterTest()
    tester.configure(1, 5)
    tester.setValue(3)
    tester.increment()
    tester.increment()
    tester.increment()
    var range = tester.getRange()
    console.log(range);
}


# TS override: return = expr syntax in event handlers
fn main() {
    var calculator = Calculator()
    var result1 = calculator.getDefault()
    console.log("Default value: " + result1);
    var result2 = calculator.calculate(10, 5)
    console.log("10 + 5 = " + String(result2));
    var result3 = calculator.divide(10, 0)
    console.log("10 / 0 = " + result3);
    var result4 = calculator.divide(10, 2)
    console.log("10 / 2 = " + String(result4));
}

system Calculator {
    interface:
        getDefault(): str 
        calculate(a: int, b: int): int
        divide(a: int, b: int)
        
    machine:
        $Ready {
            getDefault(): str {
                // Should return the default "default_value"
                system.return = "default_value";
                return;
            }
            
            calculate(a: int, b: int): int {
                // Set interface return value
                system.return = a + b;
                console.log("Calculated sum: " + String(a + b));
                return;
            }
            
            divide(a: int, b: int) {
                if (b == 0) {
                    system.return = "error: division by zero";
                    return;
                }
                system.return = a / b;
                return;
            }
        }
}


# Test: return = expr syntax in event handlers
fn main() {
    var calculator = Calculator()
    
    # Test default return value
    var result1 = calculator.getDefault()
    print("Default value: " + result1)
    
    # Test setting return value in event handler
    var result2 = calculator.calculate(10, 5)
    print("10 + 5 = " + str(result2))
    
    # Test conditional return assignment
    var result3 = calculator.divide(10, 0)
    print("10 / 0 = " + result3)
    
    var result4 = calculator.divide(10, 2)
    print("10 / 2 = " + str(result4))
}

system Calculator {
    interface:
        getDefault(): str 
        calculate(a: int, b: int): int
        divide(a: int, b: int)
        
    machine:
        $Ready {
            getDefault(): str {
                # Should return the default "default_value"
                system.return = "default_value"
                return
            }
            
            calculate(a: int, b: int): int {
                # Set interface return value
                system.return = a + b
                print("Calculated sum: " + str(a + b))
                return
            }
            
            divide(a: int, b: int) {
                if b == 0 {
                    system.return = "error: division by zero"
                    return
                }
                
                system.return = a / b
                return
            }
        }
}
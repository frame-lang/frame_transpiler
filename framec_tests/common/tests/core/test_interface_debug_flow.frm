# Test demonstrating the three-layer debugging flow:
# 1. Call site
# 2. Interface definition (NEW!)
# 3. State handler implementation

system Calculator {
    interface:
        add(a, b) : int      # Line 8 - Interface definition (should stop here)
        multiply(x, y) : int # Line 9
        divide(n, d) : float # Line 10
        
    machine:
        $Ready {
            add(a, b) : int {       # Line 14 - Handler implementation
                var result = a + b
                print("Adding: " + str(a) + " + " + str(b) + " = " + str(result))
                system.return = result
            }
            
            multiply(x, y) : int {  # Line 20
                var result = x * y
                print("Multiplying: " + str(x) + " * " + str(y) + " = " + str(result))
                system.return = result
            }
            
            divide(n, d) : float {  # Line 26
                if d == 0 {
                    print("Error: Division by zero")
                    system.return = 0.0
                } else {
                    var result = n / d
                    print("Dividing: " + str(n) + " / " + str(d) + " = " + str(result))
                    system.return = result
                }
            }
        }
}

fn main() {
    var calc = Calculator()
    
    # Debugging flow for this call:
    # 1. Stop at line 43 (call site)
    # 2. Step in → Stop at line 8 (interface definition) 
    # 3. Step in → Stop at line 14 (handler implementation)
    var sum = calc.add(5, 3)           # Line 45
    print("Sum: " + str(sum))
    
    var product = calc.multiply(4, 7)  # Line 48
    print("Product: " + str(product))
    
    var quotient = calc.divide(10, 2)  # Line 51
    print("Quotient: " + str(quotient))
}
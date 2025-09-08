// Frame v0.20 allows only main function. 
// Function-like behavior is implemented via system actions.

fn main() {
    var calc = Calculator()
    
    // Test addition
    var sum = calc.add(5, 3)
    print("5 + 3 = " + str(sum))
    
    // Test number categorization  
    var category = calc.categorizeNumber(42)
    print("42 is: " + category)
}

system Calculator {
    interface:
        add(x: int, y: int): int
        categorizeNumber(num: int): str
        
    machine:
        $Ready {
            add(x: int, y: int): int {
                system.return = x + y
                return
            }
            
            categorizeNumber(num: int): str {
                if num < 0 {
                    system.return = "negative"
                } elif num == 0 {
                    system.return = "zero"
                } elif num < 10 {
                    system.return = "single digit"
                } elif num < 100 {
                    system.return = "double digit"
                } else {
                    system.return = "large number"
                }
                return
            }
        }
}
# Test counter with domain variables and conditionals
# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION

system Counter {
    interface:
        increment()
        decrement() 
        reset()
        getValue(): i32
    
    machine:
        $Active {
            increment() {
                count = count + 1
                if count >= 10 {
                    print("Counter reached maximum")
                    -> $MaxReached
                }
            }
            
            decrement() {
                if count > 0 {
                    count = count - 1
                }
                if count == 0 {
                    print("Counter at zero")
                    -> $Zero
                }
            }
            
            reset() {
                resetCount()
                -> $Zero
            }
            
            getValue(): i32 {
                return count
            }
        }
        
        $Zero {
            increment() {
                count = 1
                -> $Active
            }
            
            getValue(): i32 {
                return 0
            }
        }
        
        $MaxReached {
            decrement() {
                count = 9
                -> $Active
            }
            
            reset() {
                resetCount()
                -> $Zero
            }
            
            getValue(): i32 {
                return 10
            }
        }
    
    actions:
        resetCount() {
            count = 0
        }
    
    domain:
        var count: i32 = 0
}
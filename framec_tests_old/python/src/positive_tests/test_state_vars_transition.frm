# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test state variable transition issue - simple two-state machine  
fn main() {
    var fsm = TwoState()
    fsm.go()
    fsm.back()
}

system TwoState {
    
    interface:
        go()
        back()
    
    machine:
        $StateA {
            var countA = 5
            
            go() {
                print("StateA: countA = " + str(countA))
                -> $StateB
            }
        }
        
        $StateB {
            var countB = 10
            
            back() {
                print("StateB: countB = " + str(countB)) 
                -> $StateA
            }
        }
}
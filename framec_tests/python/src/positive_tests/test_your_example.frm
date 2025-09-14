# Test hierarchical state machines without infinite loop
fn main() {
    var hsm = SimpleHSM()
    hsm.trigger()
    print("Test completed successfully")
}

system SimpleHSM {
    
    interface:
        trigger()
    
    machine:
        
        $Parent {
            trigger() {
                print("Parent: handling trigger, transitioning to Child")
                -> $Child
                return
            }
        }
        
        $Child => $Parent {
            $>() {
                print("Child: entered")
                # Don't transition to self - that creates infinite loop
                # Instead, just stay in Child state
            }
            
            trigger() {
                print("Child: handling trigger, transitioning back to Parent")
                -> $Parent
                return
            }
        }
}
# Test controlled HSM loop with counter and debug output
fn main() {
    var hsm = SimpleHSM()
    hsm.trigger()
    print("Done!")
}

system SimpleHSM {
    
    interface:
        trigger()
    
    machine:
        
        $Parent {
            var count = 0
            
            trigger() {
                print("Parent.trigger() called")
                count = count + 1
                print("Count is now: " + str(count))
                if count < 10 {
                    print("Transitioning to Child")
                    -> $Child
                } else {
                    print("Count >= 10, stopping")
                }
                return
            }
            
            $>() {
                print("Parent enter event")
                return
            }
        }
        
        $Child => $Parent {
            $>() {
                print("Child enter - dispatching to parent")
                => $^
                print("After parent dispatch")
                return
            }
        }
}
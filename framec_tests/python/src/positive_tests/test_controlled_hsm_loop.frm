# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test controlled HSM loop with counter
fn main() {
    var hsm = SimpleHSM()
    hsm.trigger()
}

system SimpleHSM {
    
    interface:
        trigger()
    
    machine:
        
        $Parent {
            var count = 0
            
            trigger() {
                count = count + 1
                if count < 10 {
                    -> $Child
                } else {
                    print("Count reached 10")
                }
                return
            }
        }
        
        $Child => $Parent {
            $>() {
                => $^
            }
        }
}
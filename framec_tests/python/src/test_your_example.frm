// Your example from the screenshot
fn main() {
    var hsm = SimpleHSM()
    hsm.trigger()
}

system SimpleHSM {
    
    interface:
        trigger()
    
    machine:
        
        $Parent {
            trigger() {
                -> $Child
                return
            }
        }
        
        $Child => $Parent {
            $>() {
                -> $Child
            }
        }
}
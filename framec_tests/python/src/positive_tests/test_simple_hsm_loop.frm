# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Simplest possible HSM infinite loop test
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
            }
        }
        
        $Child => $Parent {
            $>() {
                => $^
            }
        }
}
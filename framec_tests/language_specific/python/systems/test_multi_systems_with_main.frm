@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test: Multi-system with main function (v0.30)
# Validates multiple systems in multi-entity format

fn main() {
    a = 1
    sys1 = FirstSystem()
    sys1.start()
    
    sys2 = SecondSystem()
    sys2.activate()
}

system FirstSystem {
    interface:
        start()
        
    machine:
        $Idle {
            start() {
                print("FirstSystem starting")
                -> $Running
            }
        }
        
        $Running {
            $>() {
                print("FirstSystem running")
                return
            }
        }
}

system SecondSystem {
    interface:
        activate()
        
    machine:
        $Waiting {
            activate() {
                print("SecondSystem activating")
                -> $Active
            }
        }
        
        $Active {
            $>() {
                print("SecondSystem active")
                return
            }
        }
}

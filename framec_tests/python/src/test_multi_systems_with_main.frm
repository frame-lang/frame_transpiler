// Test: Multi-system with main function (v0.30)
// Validates multiple systems in multi-entity format

fn main() {
    var sys1 = FirstSystem()
    sys1.start()
    
    var sys2 = SecondSystem()
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
                return
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
                return
            }
        }
        
        $Active {
            $>() {
                print("SecondSystem active")
                return
            }
        }
}
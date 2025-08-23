// Valid multiple systems test for v0.30

fn main() {
    var first = FirstSystem()
    var second = SecondSystem()
}

system FirstSystem {
    interface:
        start()
        
    machine:
        $Begin {
            start() {
                -> $Running
            }
        }
        
        $Running {
            $>() {
                print("Running")
            }
        }
}

system SecondSystem {
    interface:
        activate()
        
    machine:
        $Idle {
            activate() {
                -> $Active
            }
        }
        
        $Active {
            $>() {
                print("Active")
            }
        }
}
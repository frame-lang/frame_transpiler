@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Valid multiple systems test for v0.30

fn main() {
    first = FirstSystem()
    second = SecondSystem()
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

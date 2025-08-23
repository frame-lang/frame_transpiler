// Two systems without main function

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
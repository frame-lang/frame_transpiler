// Test: Basic multi-system support (v0.30)
// Validates that multiple systems can be defined and transpiled

system FirstSystem {
    interface:
        start()
        
    machine:
        $Idle {
            start() {
                -> $Running
            }
        }
        
        $Running {
        }
}

system SecondSystem {
    interface:
        activate()
        
    machine:
        $Waiting {
            activate() {
                -> $Active
            }
        }
        
        $Active {
        }
}
// Test: Three systems in one module (v0.30)
// Validates multiple system definitions

system SystemA {
    interface:
        activate()
        
    machine:
        $Start {
            activate() {
                -> $Active
            }
        }
        
        $Active {
            $>() {
                print("SystemA active")
            }
        }
}

system SystemB {
    interface:
        run()
        
    machine:
        $Begin {
            run() {
                -> $Running
            }
        }
        
        $Running {
        }
}

system SystemC {
    interface:
        process()
        
    machine:
        $Initial {
            process() {
                print("Processing in SystemC")
            }
        }
}
fn main() {
    var sm = Bug50ParserFix()
    sm.start()
    sm.process()
    sm.finish()
    sm.terminate()
}

system Bug50ParserFix {
    interface:
        start()
        process()
        finish()
        terminate()
        
    machine:
        $State1 {
            start() {
                print("Starting in State1")
                -> $State2
            }
            
            process() {
                print("Processing in State1")
                -> $State3
            }
            
            terminate() {
                print("Terminating from State1")
                -> $End
            }
        }
        
        $State2 {
            start() {
                print("Already started in State2")
            }
            
            process() {
                print("Processing in State2")
                -> $State4
            }
            
            finish() {
                print("Finishing in State2")
                -> $State5
            }
            
            terminate() {
                print("Terminating from State2")
                -> $End
            }
        }
        
        $State3 {
            process() {
                print("Processing in State3")
                -> $State4
            }
            
            finish() {
                print("Finishing in State3")
                -> $State1
            }
            
            terminate() {
                print("Terminating from State3")
                -> $End
            }
        }
        
        $State4 {
            process() {
                print("Processing in State4")
                -> $State5
            }
            
            finish() {
                print("Finishing in State4")
                -> $End
            }
            
            terminate() {
                print("Terminating from State4")
                -> $End
            }
        }
        
        $State5 {
            process() {
                print("Processing in State5")
                -> $State1
            }
            
            finish() {
                print("Finishing in State5")
                -> $End
            }
            
            terminate() {
                print("Terminating from State5")
                -> $End
            }
        }
        
        $End {
            $>() {
                print("System ended successfully")
            }
        }
    
    actions:
        cleanup() {
            print("Cleanup complete")
        }
        
        reset() {
            print("Reset complete")
        }
    
    domain:
        var status = "active"
        var processCount = 0
}
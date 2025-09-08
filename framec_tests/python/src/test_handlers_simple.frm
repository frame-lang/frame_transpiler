// Simple test for event-handlers-as-functions architecture
system SimpleHandlers {
    interface:
        start()
        doWork(x)
        getState()
        
    machine:
        $Idle {
            start() {
                print("Starting work")
                -> $Working
            }
            
            getState() {
                system.return = "idle"
            }
        }
        
        $Working {
            $>() {
                print("Entered working state")
            }
            
            doWork(x) {
                print("Doing work with: " + x)
                self.workCount = self.workCount + 1
                if self.workCount >= 3 {
                    -> $Idle
                }
                system.return = "processed"
            }
            
            getState() {
                system.return = "working"
            }
        }
        
    domain:
        var workCount = 0
}

fn main() {
    var s = SimpleHandlers()
    print("State: " + s.getState())
    s.start()
    print("State: " + s.getState())
    var r1 = s.doWork("task1")
    print("Result: " + r1)
    var r2 = s.doWork("task2")
    var r3 = s.doWork("task3")
    print("State: " + s.getState())
}
# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Complex state variable test - multiple states with different variables
fn main() {
    var manager = StateManager()
    manager.start()
    manager.process()
    manager.process()
    manager.finish()
    manager.reset()
    manager.start()
}

system StateManager {
    
    interface:
        start()
        process()  
        finish()
        reset()
    
    machine:
        $Init {
            var initCount = 0
            
            start() {
                initCount = initCount + 1
                print("Starting system (attempt #" + str(initCount) + ")")
                -> $Working
            }
            
            reset() {
                print("Already in init state, resetting init count")
                initCount = 0
                return
            }
        }
        
        $Working {
            var itemsProcessed = 0
            var totalTime = 0.0
            
            process() {
                itemsProcessed = itemsProcessed + 1
                totalTime = totalTime + 2.5
                print("Processed item #" + str(itemsProcessed) + ", total time: " + str(totalTime) + "s")
                return
            }
            
            finish() {
                print("Finishing work. Processed " + str(itemsProcessed) + " items in " + str(totalTime) + "s")
                -> $Done
            }
            
            reset() {
                print("Resetting from working state")
                -> $Init
            }
        }
        
        $Done {
            var completionTime = "unknown"
            
            $>() {
                completionTime = "2024-08-28T10:30:00Z"
                print("Work completed at: " + completionTime)
                return
            }
            
            reset() {
                print("Resetting from done state (completed at: " + completionTime + ")")
                -> $Init
            }
        }
}
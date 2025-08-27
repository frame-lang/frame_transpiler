// Test: Mixed entities with complex interactions (v0.30)
// Functions, multiple systems, parameters, operations

fn main() {
    var worker = Worker("task1")
    var monitor = Monitor()
    var processor = Processor()
    
    worker.start()
    monitor.watch()
    processor.execute()
    
    shared_utility("main")
}

fn shared_utility(source) {
    print("Called from: " + source)
}

fn calculate(x, y) {
    return x * y + 5
}

system Worker(task_name) {
    interface:
        start()
        finish()
        
    machine:
        $Idle {
            start() {
                shared_utility("Worker")
                var result = calculate(10, 3)
                print("Task: " + task_name + ", Result: " + str(result))
                -> $Working
            }
        }
        
        $Working {
            finish() {
                -> $Done
            }
        }
        
        $Done {
        }
        
    domain:
        var task_name = ""
        var progress:int = 0
}

system Monitor {
    interface:
        watch()
        alert()
        
    machine:
        $Waiting {
            watch() {
                -> $Monitoring
            }
        }
        
        $Monitoring {
            alert() {
                shared_utility("Monitor")
                -> $Alerting
            }
        }
        
        $Alerting {
        }
}

system Processor {
    interface:
        execute()
        reset()
        
    machine:
        $Ready {
            execute() {
                shared_utility("Processor")
                print("Processing complete")
            }
            
            reset() {
                print("Reset processor")
            }
        }
}
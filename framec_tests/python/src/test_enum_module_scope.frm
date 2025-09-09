# Test module-scope enums

# Module-level enum (not in system)
enum GlobalStatus {
    Inactive
    Active
    Suspended
}

enum Priority : int {
    Low = 1
    Medium = 5
    High = 10
}

enum Environment : string {
    Development = "dev"
    Staging = "staging"
    Production = "prod"
}

fn main() {
    # Test module-level enum access from function
    var status = GlobalStatus.Active
    print("Global status: " + status.name)
    
    var priority = Priority.High
    print("Priority value: " + str(priority.value))
    
    var env = Environment.Production
    print("Environment: " + env.value)
    
    # Iterate module-level enum
    print("All environments:")
    for e in Environment {
        print("  - " + e.name + ": " + e.value)
    }
    
    # Use in systems
    var monitor = SystemMonitor()
    monitor.checkStatus(GlobalStatus.Active)
    monitor.setPriority(Priority.Medium)
}

system SystemMonitor {
    interface:
        checkStatus(status: GlobalStatus)
        setPriority(p: Priority)
    
    machine:
        $Monitoring {
            checkStatus(status: GlobalStatus) {
                print("System checking status: " + status.name)
                
                # Compare with module enum
                if status == GlobalStatus.Active {
                    print("Status is ACTIVE")
                } elif status == GlobalStatus.Inactive {
                    print("Status is INACTIVE")
                } else {
                    print("Status is SUSPENDED")
                }
                
                return
            }
            
            setPriority(p: Priority) {
                print("Setting priority to: " + p.name + " (value: " + str(p.value) + ")")
                
                # Iterate module enum from system
                print("Available priorities:")
                for priority in Priority {
                    if priority == p {
                        print("  > " + priority.name + " (SELECTED)")
                    } else {
                        print("    " + priority.name)
                    }
                }
                
                return
            }
        }
    
    domain:
        # System can also have its own enums
        enum InternalState {
            Init
            Ready
            Processing
            Done
        }
}
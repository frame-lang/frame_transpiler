@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Comprehensive test for explicit self/system syntax in Frame v0.31
fn main() {
    sys = ComprehensiveSystem()
    
    # Test interface calls
    sys.processTask("task1")
    sys.calculate(10, 20)
    
    # Test static method (when implemented)
    # ComprehensiveSystem.staticMethod()
}

system ComprehensiveSystem {
    operations:
        # Public operations
        publicOp() {
            print("Public operation")
        }
        
        helperOp(value: int): int {
            return value * 2
        }
    
    interface:
        processTask(taskName: string)
        calculate(a: int, b: int): int
        
    machine:
        $Ready {
            processTask(taskName: string) {
                # Test self.action() call
                self.logTask(taskName)
                
                # Test self.operation() call
                self.publicOp()
                
                # Test domain var access (will need self. when fully migrated)
                # For now, domain vars don't use self prefix in assignments
                taskCount = taskCount + 1
                
                # Test calling action with return value
                result = self.computeHash(taskName)
                print("Hash: " + str(result))
                
                # Test nested calls
                self.processInternal()
                
                return
            }
            
            calculate(a: int, b: int): int {
                # Test operation with return
                doubled = self.helperOp(a)
                
                # Test action that modifies domain
                self.updateTotal(doubled + b)
                
                system.return = total
            }
        }
        
    actions:
        # Private actions
        logTask(name: string) {
            print("Logging task: " + name)
        }
        
        computeHash(input: string): int {
            # Simple hash simulation
            return len(input) * 42
        }
        
        processInternal() {
            # Test calling another action from action
            self.internalHelper()
            
            # Test calling operation from action
            self.publicOp()
        }
        
        internalHelper() {
            print("Internal helper called")
        }
        
        updateTotal(value: int) {
            total = total + value
            print("Total updated to: " + str(total))
        }
        
    domain:
        taskCount: int = 0
        total: int = 0
}

# Test multiple systems with self context
system SecondarySystem {
    interface:
        test()
        
    machine:
        $Start {
            test() {
                # Each system has its own self context
                self.ownAction()
                return
            }
        }
        
    actions:
        ownAction() {
            print("SecondarySystem action")
        }
}

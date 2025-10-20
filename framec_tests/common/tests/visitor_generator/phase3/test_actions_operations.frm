# Phase 3: Actions and operations

system Calculator {
    operations:
        getVersionString(): string {
            return "Calculator v1.0"
        }
    
    interface:
        add(x: int, y: int): int
        multiply(a: float, b: float): float
        getVersion(): string
    
    machine:
        $Active {
            add(x: int, y: int) {
                var result = performAdd(x, y)
                system.return = result
                return
            }
            
            multiply(a: float, b: float) {
                var result = performMultiply(a, b)
                system.return = result
                return
            }
            
            getVersion() {
                system.return = getVersionString()
                return
            }
        }
    
    actions:
        performAdd(x: int, y: int): int {
            return x + y
        }
        
        performMultiply(a: float, b: float): float {
            return a * b
        }
}
# Phase 2: Multiple interface methods with parameter handling

system MathProcessor {
    interface:
        add(x: int, y: int): int
        multiply(a: float, b: float): float
        greet(name: string): string

    machine:
        $Ready {
            add(x: int, y: int) {
                system.return = x + y
                return
            }
            
            multiply(a: float, b: float) {
                system.return = a * b
                return
            }
            
            greet(name: string) {
                system.return = "Hello " + name
                return
            }
        }
}
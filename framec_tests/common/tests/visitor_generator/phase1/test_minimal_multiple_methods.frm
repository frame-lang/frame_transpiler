# Phase 1 Test: Multiple Interface Methods
# Tests a system with multiple interface methods
# This validates:
# - Multiple interface method declarations
# - Method dispatch to correct handlers
# - Different parameter counts
# - String concatenation operations
#
# Expected outputs:
# - greeting("World") -> "Hello, World!"
# - farewell() -> "Goodbye!"

system Greeter {
    interface:
        greeting(name: string): string
        farewell(): string
    
    machine:
        $Active {
            greeting(name: string): string {
                system.return = "Hello, " + name + "!"
                return
            }
            
            farewell(): string {
                system.return = "Goodbye!"
                return
            }
        }
}
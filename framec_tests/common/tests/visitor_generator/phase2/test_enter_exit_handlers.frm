# Phase 2: Enter handlers (exit handlers appear to have parser issues)

system LifecycleMachine {
    interface:
        activate(): string
        deactivate(): string

    machine:
        $Inactive {
            $>() {
                print("Entering inactive state")
            }
            
            activate() {
                system.return = "Activated"
                -> $Active
            }
        }
        
        $Active {
            $>() {
                print("Entering active state")
            }
            
            deactivate() {
                system.return = "Deactivated"
                -> $Inactive
            }
        }
}
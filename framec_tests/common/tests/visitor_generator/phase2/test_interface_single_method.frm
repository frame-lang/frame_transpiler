# Phase 2: Single interface method with basic state machine

system Calculator {
    interface:
        add(x: int, y: int): int

    machine:
        $Begin {
            add(x: int, y: int) {
                system.return = x + y
                return
            }
        }
}
@target python_3

# V3 capability: system.return from handlers and actions.

system SystemReturnDemo {
    interface:
        status(): str = "idle"

    machine:
        $Idle {
            status() {
                # Allowed: handlers can assign system.return
                system.return = "idle"
            }
        }

    actions:
        set_status(value) {
            # Allowed: actions can assign system.return
            system.return = value
        }
}

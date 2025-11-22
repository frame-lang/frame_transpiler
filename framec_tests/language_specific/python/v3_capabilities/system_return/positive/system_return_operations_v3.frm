@target python_3

# V3 capability: system.return from operations (Python).
# Operations are allowed to assign to system.return, sharing the same
# per-call slot used by handlers and actions.

system SystemReturnOpDemo {
    interface:
        status(): str = "idle"
        get_default(): str = "op-default"

    machine:
        $Idle {
            status() {
                # Handler assigns system.return explicitly.
                system.return = "idle"
            }

            get_default() {
                # Delegate to an operation that assigns system.return.
                self.compute_default()
            }
        }

    operations:
        compute_default() {
            system.return = "op-default"
        }
}


@target rust

// V3 capability: system.return from handlers and actions (Rust).
//
// Mirrors the Python/TypeScript fixtures for handlers/actions setting
// system.return inside a Rust-target system.

system SystemReturnHandlersActionsRs {
    interface:
        status(): String = String::from("idle")

    machine:
        $Idle {
            status() {
                // Allowed: handlers can assign system.return.
                system.return = String::from("idle");
            }
        }

    actions:
        set_status(value: String) {
            // Allowed: actions can assign system.return.
            system.return = value;
        }
}


@target typescript

# V3 capability: system.return from handlers and actions (TypeScript).

system SystemReturnDemoTs {
    interface:
        status(): string = "idle"

    machine:
        $Idle {
            status() {
                system.return = "idle";
            }
        }

    actions:
        setStatus(value: string) {
            system.return = value;
        }
}

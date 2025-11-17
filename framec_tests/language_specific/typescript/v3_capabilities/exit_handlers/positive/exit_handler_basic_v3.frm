@target typescript

# V3 capability fixture: exit handler (<$()) support (TypeScript).
# Structural / compile-only: exercises $>() and <$() headers in a simple system.

system ExitHandlerDemoTs {
    interface:
        tick()

    machine:
        $A {
            $>() {
                console.log("enter A");
            }

            tick() {
                -> $B()
            }

            <$() {
                console.log("exit A");
            }
        }

        $B {
            $>() {
                console.log("enter B");
            }
        }
}


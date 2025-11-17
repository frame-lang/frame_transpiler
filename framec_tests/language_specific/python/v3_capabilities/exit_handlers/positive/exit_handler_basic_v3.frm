@target python_3

# V3 capability fixture: exit handler (<$()) support.
# Structural / compile-only: exercises $>() and <$() headers in a simple system.

system ExitHandlerDemo {
    interface:
        tick()

    machine:
        $A {
            $>() {
                print("enter A")
            }

            tick() {
                -> $B()
            }

            <$() {
                print("exit A")
            }
        }

        $B {
            $>() {
                print("enter B")
            }
        }
}


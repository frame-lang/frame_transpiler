@target python_3

# V3 capability: system.method() calling an interface method.

system SystemCallDemo {
    interface:
        status()

    machine:
        $A {
            status() {
                system.return = "ok"
            }

            e() {
                system.status()
            }
        }
}


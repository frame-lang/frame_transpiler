@target typescript

# V3 capability: system.method() calling an interface method (TypeScript).

system SystemCallDemoTs {
    interface:
        status()

    machine:
        $A {
            status() {
                system.return = "ok";
            }

            e() {
                system.status();
            }
        }
}


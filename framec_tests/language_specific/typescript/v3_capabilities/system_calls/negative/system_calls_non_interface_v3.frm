@target typescript
# @expect: E406

# Negative: system.helper() is not an interface method.

system BadSystemCallTs {
    interface:
        status()

    actions:
        doThing() {
            system.helper();
        }

    machine:
        $A {
            e() {
                system.helper();
            }
        }
}


@target python_3
# @expect: E406

# Negative: system.helper() is not an interface method.

system BadSystemCall {
    interface:
        status()

    actions:
        doThing() {
            system.helper()
        }

    machine:
        $A {
            e() {
                system.helper()
            }
        }
}


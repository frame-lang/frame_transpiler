@target rust
// @expect: E406

// Negative: system.helper() is not an interface method.

system BadSystemCallRs {
    interface:
        status()

    actions:
        do_thing() {
            system.helper();
        }

    machine:
        $A {
            e() {
                system.helper();
            }
        }
}


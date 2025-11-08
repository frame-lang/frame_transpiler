system TestSys2 {
    interface:
        go()

    machine:
        $A {
            go() {
                helper()
                return
            }
        }

    operations:
        helper() {
            // Not allowed: transitions in operations
            -> $A
        }
}


system TestSys {
    interface:
        go()

    machine:
        $A {
            go() {
                doWork()
                return
            }
        }

    actions:
        doWork() {
            // Not allowed: transitions in actions
            -> $A
        }
}


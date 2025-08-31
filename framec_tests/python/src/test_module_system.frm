system TestSystem {
    interface:
        start()

    machine:
        $Begin {
            start() {
                print("System started")
            }
        }
}

var mySystem = TestSystem()

mySystem.start()
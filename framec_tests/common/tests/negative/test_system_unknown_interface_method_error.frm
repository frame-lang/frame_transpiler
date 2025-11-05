# Negative: calling a non-existent interface method via system.method()

system UnknownInterfaceMethodTest {
    interface:
        test()

    machine:
        $Start {
            test() {
                var result = system.nonexistentMethod()
                return
            }
        }
}


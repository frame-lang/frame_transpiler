# Negative: nested member access under system is not supported (system.foo.bar())

system NestedSystemMemberTest {
    interface:
        test()

    machine:
        $Start {
            test() {
                var x = system.foo.bar()
                return
            }
        }
}


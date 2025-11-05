# Negative: bare 'system' keyword is invalid (only system.return is allowed)

system BareSystemKeywordTest {
    interface:
        test()

    machine:
        $Start {
            test() {
                var tmp = system
                return
            }
        }
}


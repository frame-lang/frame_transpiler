# TS override: transition return

system TransitionReturnTest {
    interface:
        go()

    machine:
        $Start {
            go() { -> $End }
        }
        $End { }
}


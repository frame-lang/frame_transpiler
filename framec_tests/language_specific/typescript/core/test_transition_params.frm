# TypeScript-specific copy of common fixture

system TransitionParamsSm {
    interface:
        Init()
        Go(val:int)
        Report()

    machine:
        $A {
            Init() { -> (1) $B }
            Go(val:int) { -> (val) $B }
            Report() { return }
        }
        $B(n:int) {
            Report() { print("n=" + str(n)); return }
        }

    domain:
        var tape = []
}


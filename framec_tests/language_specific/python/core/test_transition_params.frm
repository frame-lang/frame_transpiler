@target python

# Python-specific: domain native assignment

system TransitionParamsSm {
    interface:
        Init()
        Go(val:int)
        Report()

    machine:
        $A {
            Init() { -> $B(1) }
            Go(val:int) { -> $B(val) }
            Report() { return }
        }
        $B(n:int) {
            Report() {
                print("n=" + str(n))
                return
            }
        }

    domain:
        tape = []
}

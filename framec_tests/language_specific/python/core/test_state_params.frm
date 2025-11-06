# Python-specific: domain native assignment

system StateParamsSm {
    interface:
        Next()
        Prev()
        Report()

    machine:
        $Init {
            Next() { -> (1) $S(0) }
            Prev() { return }
            Report() { return }
        }
        $S(k:int) {
            Next() { -> (k+1) $S(k) }
            Prev() { -> $Init }
            Report() { print("k=" + str(k)); return }
        }

    domain:
        param_log = []
}


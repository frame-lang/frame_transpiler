@target python

# Negative: unterminated single-quoted string in Python native body

system BadSingleQuote {
    interface:
        Run()

    machine:
        $S {
            Run() {
                msg = 'unterminated
                return
            }
        }
}


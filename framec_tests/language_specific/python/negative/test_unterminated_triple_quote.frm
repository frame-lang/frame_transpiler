@target python

# Negative: unterminated triple-quoted string in Python native body

system BadTripleQuote {
    interface:
        Run()

    machine:
        $S {
            Run() {
                print("start")
                s = """unterminated triple quote...
                // Missing closing triple quote and body brace alignment should error
            }
        }
}


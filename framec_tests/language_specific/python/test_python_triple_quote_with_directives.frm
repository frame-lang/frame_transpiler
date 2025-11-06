@target python

system PyTripleQuotes {
    actions:
        run() {
            doc = """
            This triple-quoted string contains tokens that must not segment:
            -> $Next
            $$[+]
            => $^
            """
            x = len(doc)
    machine:
        $Init { run() { } }
        $Next {}
}


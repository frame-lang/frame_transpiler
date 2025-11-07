
system TripleQuotesPy {
    machine:
        $Init {
            start() {
                text = '''Line mentions -> $Next but should be ignored'''
                print(text)
                -> $Next
            }
        }
        $Next {}
}

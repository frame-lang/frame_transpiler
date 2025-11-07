
system TripleQuotesMultilinePy {
    machine:
        $Init {
            start() {
                text = '''
This string mentions => $^ and $$[-] and should not be segmented.
It also mentions -> $Other within the string.
'''
                print(text)
                -> $Next
            }
        }
        $Next {}
}

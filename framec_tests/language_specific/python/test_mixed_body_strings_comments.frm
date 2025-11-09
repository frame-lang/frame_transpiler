
system MixedBodyStringsComments {
    machine:
        $Init {
            start() {
                # Native Python with Frame-statement-like tokens in strings and comments
                text = "This mentions -> $Next and $$[-] inside a string."
                # A comment that mentions => $^ and -> $Other should not be parsed as Frame
                print(text)
                -> $Done
            }
        }
        $Done {}
}

 
system CommentsWithDirectiveTokens {
    interface:
        start()

    machine:
        $Init {
            start() {
                // This line contains a Frame-statement-like token: -> $Fake but is a comment
                const a = 1; // -> $NotADirective
                /* Multi-line comment with tokens
                   => $^
                   $$[+]
                */
                const b = 2;
                return
            }
        }
    actions:
    domain:
}

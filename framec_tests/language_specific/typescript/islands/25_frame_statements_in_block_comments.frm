 

system BlockCommentDirectivesTS {
    actions:
        run() {
            /*
             * The following tokens must NOT segment:
             * -> $Next
             * $$[+]
             * => $^
             */
            const z = 42;
        }
    machine:
        $Init { run() { } }
        $Next {}
}

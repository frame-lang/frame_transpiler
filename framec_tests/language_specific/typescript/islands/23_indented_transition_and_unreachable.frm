@target typescript

system IndentedTransitionTS {
    actions:
        run() {
            // Indented directive at SOL (first non-whitespace)
            const a = 1;
            	-> $Next
            const b = 2; // should be preserved with a single unreachable warning
            const c = a + b;
        }
    machine:
        $Init { run() { } }
        $Next {}
}


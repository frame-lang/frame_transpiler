fn incomplete_function(
    // Missing closing paren and brace - tests line number reporting

system BrokenSystem {
    machine:
        $State {
            handler() {
                // Missing closing braces for handler, state, machine, and system
                // Tests multiple syntax error reporting
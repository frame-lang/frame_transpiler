@target rust

system S {
    actions:
        bad() {
            -> $B()
        }
}


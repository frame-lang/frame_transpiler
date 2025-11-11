@target rust

system S {
    actions:
        fn bad() {
            -> $B()
        }
}

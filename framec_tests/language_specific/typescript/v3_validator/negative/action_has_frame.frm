@target typescript

system S {
    actions:
        fn bad() {
            -> $B()
        }
}

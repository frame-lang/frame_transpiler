@target c

system S {
    actions:
        fn bad() {
            -> $B()
        }
}

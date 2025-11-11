@target cpp

system S {
    actions:
        fn bad() {
            -> $B()
        }
}

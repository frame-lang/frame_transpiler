@target python

system S {
    actions:
        fn bad() {
            -> $B()
        }
}

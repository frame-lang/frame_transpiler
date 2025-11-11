@target java

system S {
    actions:
        fn bad() {
            -> $B()
        }
}

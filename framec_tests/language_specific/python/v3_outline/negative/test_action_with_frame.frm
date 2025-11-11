@target python

system S {
    actions:
        fn do_it() {
            -> $Bad()
        }
}

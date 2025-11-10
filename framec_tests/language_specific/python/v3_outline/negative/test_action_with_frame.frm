@target python

system S {
    actions:
        do_it() {
            -> $Bad()
        }
}

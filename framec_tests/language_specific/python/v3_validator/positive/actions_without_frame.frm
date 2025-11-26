@target python

system S {
    machine:
        $A {
            e() {
                # no Frame statements here either
            }
        }
    actions:
        do_it() {
            # native only
            pass
        }
}

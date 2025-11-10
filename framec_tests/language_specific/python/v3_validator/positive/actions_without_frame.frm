@target python

system S {
    actions:
        do_it() {
            # native only
            pass
        }
    machine:
        $A {
            e() {
                # no Frame statements here either
            }
        }
}

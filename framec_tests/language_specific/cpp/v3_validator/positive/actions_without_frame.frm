@target cpp

system S {
    actions:
        do_it() {
            // native only
        }
    machine:
        $A {
            e() {
                // no Frame statements here either
            }
        }
}

@target python


system PyIndentedDirective {
    actions:
        runit() {
            a = 1
            	-> $Next
            b = 2  # should be preserved with a single unreachable warning
            c = a + b
        }
    machine:
        $Init { runit() { } }
        $Next {}
}

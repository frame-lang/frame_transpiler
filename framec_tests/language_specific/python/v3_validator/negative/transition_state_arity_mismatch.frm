@target python
# @expect: E405

system S {
    machine:
        $A {
            e() {
                -> $B(1)
            }
        }
        $B(x, y) { }
}


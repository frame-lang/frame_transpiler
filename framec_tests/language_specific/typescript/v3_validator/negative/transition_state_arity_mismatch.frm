@target typescript
// @expect: E405

system S {
    machine:
        $A {
            e() {
                -> $B(1, 2, 3)
            }
        }
        $B(x, y) { }
}


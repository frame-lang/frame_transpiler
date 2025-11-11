@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
system T {
    machine:
        $A {
            go() {
                -> $B
            }
        }
        $B {}
}

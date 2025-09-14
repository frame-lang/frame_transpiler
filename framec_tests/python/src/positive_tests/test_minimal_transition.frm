system T {
    machine:
        $A {
            go() {
                -> $B
                return
            }
        }
        $B {}
}
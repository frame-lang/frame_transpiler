@target c

system S {
    machine:
        $A {
            e() {
                /* line1 with -> $B()
                   line2 with => $^
                 */
                native();
            }
        }
}


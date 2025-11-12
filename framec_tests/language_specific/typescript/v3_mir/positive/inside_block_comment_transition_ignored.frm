@target typescript

system S {
    machine:
        $A {
            e() {
                /* block with -> $B() and => $^ */
                native();
            }
        }
}


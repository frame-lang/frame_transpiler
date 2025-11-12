@target rust

system S {
    machine:
        $A {
            e() {
                /* nested block comment with -> $B() and => $^ */
                native();
            }
        }
}


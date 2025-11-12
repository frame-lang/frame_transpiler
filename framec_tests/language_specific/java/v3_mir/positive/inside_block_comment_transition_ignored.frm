@target java

system S {
    machine:
        $A {
            e() {
                /* block comment with -> $B() and => $^ */
                native();
            }
        }
}


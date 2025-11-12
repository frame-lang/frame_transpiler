@target java

system S {
    machine:
        $A {
            e() {
                // comment with -> $B() and => $^
                native();
            }
        }
}


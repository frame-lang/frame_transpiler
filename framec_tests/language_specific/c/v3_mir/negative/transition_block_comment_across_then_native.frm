@target c

system S {
    machine:
        $A {
            e() {
                -> $B() /* start
                   still comment
                   */ native();
            }
        }
}

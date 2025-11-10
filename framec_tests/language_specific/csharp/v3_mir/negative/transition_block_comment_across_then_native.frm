@target csharp

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

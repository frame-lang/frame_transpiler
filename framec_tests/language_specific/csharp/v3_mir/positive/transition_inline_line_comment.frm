@target csharp

system S {
    machine:
        $A {
            e() {
                -> $B() // comment ok
            }
        }
        $B {
        }
}

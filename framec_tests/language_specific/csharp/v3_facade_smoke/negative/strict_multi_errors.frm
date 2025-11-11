@target csharp

system S {
    machine:
        $A {
            e() {
                -> $B();
                List<int x;     // missing '>'
                int y = ;       // missing initializer
                if ( { ) { }    // malformed if
            }
        }
        $B {
        }
}


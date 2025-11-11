@target c

system S {
    machine:
        $A {
            e() {
                -> $B();
                int x = ;      // missing initializer
                if ( { ) { }   // malformed if
            }
        }
        $B {
        }
}


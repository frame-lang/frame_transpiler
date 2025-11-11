@target java

system S {
    machine:
        $A {
            e() {
                -> $B();
                List<String x;   // missing '>'
                int y = ;        // missing initializer
                if ( { ) { }     // malformed if
            }
        }
        $B {
        }
}


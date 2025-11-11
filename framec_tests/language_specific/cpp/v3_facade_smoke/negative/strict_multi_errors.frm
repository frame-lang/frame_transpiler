@target cpp

system S {
    machine:
        $A {
            e() {
                -> $B();
                std::vector<int x;    // missing '>'
                int y = ;             // missing initializer
                if ( { ) { }          // malformed if
            }
        }
        $B {
        }
}


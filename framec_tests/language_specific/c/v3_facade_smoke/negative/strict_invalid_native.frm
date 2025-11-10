@target c

system S {
    machine:
        $A {
            e() {
                -> $B();
                int x = ;   // malformed native statement for strict facade
            }
        }
        $B {
        }
}


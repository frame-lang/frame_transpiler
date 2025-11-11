@target python

system S {
    machine:
        $A {
            e() {
                -> $B(1, "x")
                x =   # malformed native statement for strict facade
            }
        }
        $B {
        }
}


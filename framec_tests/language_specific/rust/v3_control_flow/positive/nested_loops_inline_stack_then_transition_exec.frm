@target rust

system S {
    machine:
        $A {
            e() {
                {
                    // nested block as a stand-in for loops
                    $$[+]
                    { $$[-] }
                }
                -> $B()
            }
        }
        $B { e() { } }
}


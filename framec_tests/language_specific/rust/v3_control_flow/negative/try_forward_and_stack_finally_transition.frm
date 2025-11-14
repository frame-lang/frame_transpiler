@target rust
// @expect: E403

system S {
    machine:
        $A {
            e() {
                // Block as a stand-in for try; forward without parent should fail
                {
                    => $^
                    $$[+]
                }
                $$[-]
                -> $B()
            }
        }
        $B { e() { } }
}


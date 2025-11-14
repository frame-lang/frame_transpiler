@target rust

system S {
    machine:
        $A {
            e() {
                // use a non-terminal stack op; inline after is allowed
                $$[+]
                let z = 10; // inline after non-terminal op
                -> $B()
            }
        }
        $B { e() { } }
}

@target typescript

system S {
    machine:
        $A {
            e() {
                // Native parse (SWC) should flag missing initializer
                const x = ;
                -> $B()
            }
        }
}


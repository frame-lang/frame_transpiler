@target typescript

system S {
    machine:
        $A {
            e() {
                -> $B();
                let a: Array<number = []; // missing '>'
                let b = ;                 // missing initializer
                if ({) { }                // malformed if condition
            }
        }
        $B {
        }
}


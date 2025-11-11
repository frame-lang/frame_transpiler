@target rust

system S {
    machine:
        $A {
            e() {
                -> $B();
                let v: Vec<i32 = vec![1,2]; // missing '>'
                let z = ;                    // missing initializer
            }
        }
        $B {
        }
}


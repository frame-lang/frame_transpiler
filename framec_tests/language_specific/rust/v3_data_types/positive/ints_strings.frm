@target rust

system S {
    machine:
        $A {
            e() { let mut n:i32=1; let s:&str = "x"; => $^; n = n + 1; }
        }
}


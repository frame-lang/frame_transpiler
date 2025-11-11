@target rust

system S {
    machine:
        $A => $P {
            e() { let mut n:i32=1; let s:&str = "x"; => $^; n = n + 1; }
        }
        $P { }
}

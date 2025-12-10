@target rust

system S {
    machine:
        $A => $P {
            e() {
                let mut v: Vec<i32> = vec![1, 2, 3];
                v.push(4);
                let first = v.get(0).copied().unwrap_or(0);
                if first > 0 {
                    v.remove(0);
                }
                => $^;
            }
        }
        $P { }
}

@target rust

system S {
    machine:
        $A => $P {
            e() {
                let t: (i32, &str, bool) = (1, "x", true);
                let (a, b, c) = t;
                let _sum = a + if c { 1 } else { 0 };
                let _len = b.len();
                => $^;
            }
        }
        $P { }
}

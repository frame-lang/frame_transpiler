@target rust

system S {
    machine:
        $A => $P {
            e() {
                use std::collections::HashSet;
                let mut s: HashSet<&str> = HashSet::new();
                s.insert("a");
                s.insert("b");
                s.insert("a");
                s.remove("b");
                let _contains = s.contains("a");
                => $^;
            }
        }
        $P { }
}

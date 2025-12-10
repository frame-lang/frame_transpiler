@target rust

system S {
    machine:
        $A => $P {
            e() {
                use std::collections::HashMap;
                let mut m: HashMap<String, i32> = HashMap::new();
                m.insert("one".to_string(), 1);
                m.insert("two".to_string(), 2);
                if let Some(v) = m.get_mut("one") {
                    *v += 1;
                }
                let _size = m.len();
                => $^;
            }
        }
        $P { }
}

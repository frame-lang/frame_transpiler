@target rust

system S {
    machine:
        $A {
            e() {
                let s = r#"raw string -> $B() ignored"#;
                a();
            }
        }
}


@target rust

system S {
    machine:
        $A {
            e() {
                // Invalid Rust native statement to trigger strict parser
                let bad = ;
                => $^;
            }
        }
}


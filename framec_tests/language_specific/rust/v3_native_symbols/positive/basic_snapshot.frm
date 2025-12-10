@target rust

system S {
    machine:
        $A {
            e(x: i32, y: &str) {
                native()
            }
        }
}

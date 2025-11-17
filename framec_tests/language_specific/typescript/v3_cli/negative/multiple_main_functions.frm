@target typescript

// Multiple top-level main functions. Expect E115.

fn main() {
    // first entry point
}

fn main() {
    // second entry point (invalid)
}

system S {
    machine:
        $A { e() { x(); } }
}


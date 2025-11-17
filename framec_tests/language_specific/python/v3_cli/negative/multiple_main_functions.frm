@target python_3

# Multiple top-level main functions. Expect E115.

fn main() {
    # first entry point
    pass
}

fn main() {
    # second entry point (invalid)
    pass
}

system S {
    machine:
        $A { e() { x() } }
}


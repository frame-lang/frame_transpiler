// EXPECTED_OUTPUT: NoParameters started
fn main() {
    var sys = NoParameters()
    // System should auto-start with enter event now
}

system NoParameters {
    machine:
        $Start {
            $>() {
                print("NoParameters started")
                return
            }
        }
}
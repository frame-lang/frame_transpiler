@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# EXPECTED_OUTPUT: NoParameters started
fn main() {
    sys = NoParameters()
    # System should auto-start with enter event now
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

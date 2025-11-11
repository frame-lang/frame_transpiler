@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# EXPECTED_OUTPUT: started
fn main() {
    sys = NoParamsSystem()
}

system NoParamsSystem {
    machine:
        $Start {
            $>() {
                print("started")
                return
            }
        }
}

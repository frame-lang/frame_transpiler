// EXPECTED_OUTPUT: started
fn main() {
    var sys = NoParamsSystem()
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
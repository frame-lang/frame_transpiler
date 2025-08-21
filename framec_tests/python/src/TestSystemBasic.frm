// Test basic system with no parameters
fn main() {
    var sys = NoParamsSystem()
}

system NoParamsSystem {
    machine:
        $Start {
            $>() {
                print("NoParamsSystem started")
                return
            }
        }
}
# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    var sys = TestSystem()
    helper()
}

fn helper() {
    print("Helper function called")
}

system TestSystem {
    machine:
    
    $Init {
        $>() {
            print("System initialized")
            return
        }
    }
}
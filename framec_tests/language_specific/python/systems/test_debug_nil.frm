# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    print("Testing")
}

system TestSystem {
    machine:
        $Start {
            testVar = None
            
            test() {
                print("In test")
            }
        }
}
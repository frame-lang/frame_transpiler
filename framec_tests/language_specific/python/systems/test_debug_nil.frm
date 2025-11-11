@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    print("Testing")
}

system TestSystem {
    machine:
        $Start {
            var testVar = None
            
            test() {
                print("In test")
            }
        }
}

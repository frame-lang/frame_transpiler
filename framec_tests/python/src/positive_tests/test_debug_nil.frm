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
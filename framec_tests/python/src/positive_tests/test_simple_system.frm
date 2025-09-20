# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    var sys = SimpleSystem()
    sys.test()
}

system SimpleSystem {
    interface:
        test()
        
    machine:
        $Start {
            test() {
                print("Testing")
            }
        }
}
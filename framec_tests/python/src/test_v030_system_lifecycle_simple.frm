// Simplified system lifecycle test to verify basic functionality

fn main() {
    print("Starting simple lifecycle test")
    var sys = SimpleSystem()
    sys.test()
    print("Test complete")
}

system SimpleSystem {
    interface:
        test()
        
    machine:
        $Start {
            $>() {
                print("Entering Start")
            }
            
            test() {
                print("In test()")
                -> $End
            }
        }
        
        $End {
            $>() {
                print("Entering End")
            }
        }
}
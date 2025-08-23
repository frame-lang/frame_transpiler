// Test: System without function

system OnlySystem {
    interface:
        test()
        
    machine:
        $Start {
            $>() {
                print("Entering Start")
            }
            
            test() {
                print("In test method")
                -> $End
            }
        }
        
        $End {
            $>() {
                print("Entering End")
            }
        }
}
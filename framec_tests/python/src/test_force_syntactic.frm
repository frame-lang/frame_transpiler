// Force syntactic parsing by having 2 systems

system Dummy {
    machine:
        $S {
        }
}

system TestSystem {
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
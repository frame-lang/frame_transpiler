@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Single system test to verify code generation

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

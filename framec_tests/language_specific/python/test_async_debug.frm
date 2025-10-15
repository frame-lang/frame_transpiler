# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION

# Debug test for async interface methods
system AsyncDebug {
    interface:
        async getData(id)
        normalMethod(x)
        
    machine:
        $Ready {
            getData(id) {
                print("ID: " + str(id))
                -> $Processing
            }
            
            normalMethod(x) {
                print("X: " + str(x))
                return x
            }
        }
        
        $Processing {
            $>() {
                print("In processing")
            }
        }
}
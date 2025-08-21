// Minimal test case for transition + return in if/elif/else
fn main() {
    var sys = TransitionTest()
    sys.test()
}

system TransitionTest {
    interface:
        test()
        
    machine:
        $Start {
            test() {
                var condition = "error"
                
                if condition == "error" {
                    -> $Error
                    return
                } elif condition == "success" {
                    -> $Success
                    return
                } else {
                    -> $Default
                    return
                }
            }
        }
        
        $Error {
            $>() {
                print("In error state")
                return
            }
        }
        
        $Success {
            $>() {
                print("In success state")
                return
            }
        }
        
        $Default {
            $>() {
                print("In default state")
                return
            }
        }
}
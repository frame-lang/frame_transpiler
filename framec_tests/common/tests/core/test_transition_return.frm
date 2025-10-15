# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Minimal test case for transition + return in if/elif/else
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
                } elif condition == "success" {
                    -> $Success
                } else {
                    -> $Default
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
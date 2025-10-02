# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    var sys = TestContinue()
    sys.test()
}

system TestContinue {
    interface:
        test()
    
    machine:
        $Child => $Parent {
            test() {
                print("in child")
                => $^
            }
        }
        
        $Parent {
            test() {
                print("in parent")
                return
            }
        }
}
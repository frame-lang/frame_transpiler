fn main() {
    var sys = TestForwardEvent()
    sys.test()
}

system TestForwardEvent {
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
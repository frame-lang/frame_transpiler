// Test: Single system lifecycle to isolate the issue

fn main() {
    print("=== Single System Test ===")
    var sys = SingleSystem()
    sys.next()
    sys.next()
    sys.next()
    print("=== Test Complete ===")
}

system SingleSystem {
    interface:
        next()
        
    machine:
        $Start {
            $>() {
                print("Entering Start")
            }
            
            <$() {
                print("Exiting Start")
            }
            
            next() {
                print("Start.next() -> Working")
                return = true
                -> $Working
            }
        }
        
        $Working {
            $>() {
                print("Entering Working")
            }
            
            <$() {
                print("Exiting Working")
            }
            
            next() {
                print("Working.next() -> End")
                return = true
                -> $End
            }
        }
        
        $End {
            $>() {
                print("Entering End")
            }
            
            <$() {
                print("Exiting End")
            }
            
            next() {
                print("End.next() - complete")
                return = false
            }
        }
}
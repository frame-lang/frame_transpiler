import json

fn main() {
    var sys = TestSystem()
    sys.start()
}

system TestSystem {
    interface:
        start()
    
    machine:
        $Init {
            start() {
                -> $Running
            }
        }
        
        $Running {
            $>() {
                print("Running state entered")
            }
        }
    
    actions:
        helper_action() {
            print("Helper called")
        }
    
    domain:
        var counter = 0
}
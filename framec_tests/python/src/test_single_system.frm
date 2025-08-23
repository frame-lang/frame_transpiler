// Single system test for comparison

fn main() {
    var first = FirstSystem()
}

system FirstSystem {
    interface:
        start()
        
    machine:
        $Begin {
            start() {
                -> $Running
            }
        }
        
        $Running {
            $>() {
                print("Running")
            }
        }
}
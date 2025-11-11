@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Single system test for comparison

fn main() {
    first = FirstSystem()
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

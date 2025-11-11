@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# First system only

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

@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    sys1 = SystemA()
    sys1.start()
}

system SystemA {
    interface:
        start()
        
    machine:
        $Idle {
            $>() {
                return
            }
            start() {
                -> $Running
            }
        }
        
        $Running {
            $>() {
                return
            }
        }
}

system SystemB {
    interface:
        activate()
        
    machine:
        $Waiting {
            $>() {
                return
            }
            activate() {
                -> $Active
            }
        }
        
        $Active {
            $>() {
                return
            }
        }
}
